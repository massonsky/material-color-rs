use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;

use material_color::TemperatureCache;
use material_color::argb::Argb;
use material_color::blend;
use material_color::cam::Hct;
use material_color::contrast;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};
use material_color::palettes::TonalPalette;
use material_color::quantize;
use material_color::score::{ScoreOptions, ranked_suggestions};
use material_color::utils;

const VERSION_MAJOR: u32 = parse_version_component(env!("CARGO_PKG_VERSION_MAJOR"));
const VERSION_MINOR: u32 = parse_version_component(env!("CARGO_PKG_VERSION_MINOR"));
const VERSION_PATCH: u32 = parse_version_component(env!("CARGO_PKG_VERSION_PATCH"));

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialColorStatus {
    Ok = 0,
    InvalidArgument = 1,
    Panic = 255,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MaterialColorHct {
    pub hue: f64,
    pub chroma: f64,
    pub tone: f64,
    pub argb: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MaterialColorPopulation {
    pub argb: u32,
    pub population: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MaterialColorArgbArray {
    pub data: *mut u32,
    pub len: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MaterialColorPopulationArray {
    pub data: *mut MaterialColorPopulation,
    pub len: usize,
}

fn ffi_guard(function: impl FnOnce() -> MaterialColorStatus) -> MaterialColorStatus {
    match catch_unwind(AssertUnwindSafe(function)) {
        Ok(status) => status,
        Err(_) => MaterialColorStatus::Panic,
    }
}

fn write_out<T>(out: *mut T, value: T) -> MaterialColorStatus {
    if out.is_null() {
        return MaterialColorStatus::InvalidArgument;
    }

    unsafe {
        out.write(value);
    }
    MaterialColorStatus::Ok
}

fn slice_from_raw<'a, T>(data: *const T, len: usize) -> Option<&'a [T]> {
    if len == 0 {
        Some(&[])
    } else if data.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(data, len) })
    }
}

fn argb_array_from_vec(values: Vec<u32>) -> MaterialColorArgbArray {
    if values.is_empty() {
        return MaterialColorArgbArray::default();
    }

    let len = values.len();
    let boxed = values.into_boxed_slice();
    let data = Box::into_raw(boxed).cast::<u32>();

    MaterialColorArgbArray { data, len }
}

fn population_array_from_map(values: BTreeMap<Argb, u32>) -> MaterialColorPopulationArray {
    let entries = values
        .into_iter()
        .map(|(argb, population)| MaterialColorPopulation {
            argb: argb.to_u32(),
            population,
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        return MaterialColorPopulationArray::default();
    }

    let len = entries.len();
    let boxed = entries.into_boxed_slice();
    let data = Box::into_raw(boxed).cast::<MaterialColorPopulation>();

    MaterialColorPopulationArray { data, len }
}

fn variant_from_i32(value: i32) -> Option<Variant> {
    match value {
        0 => Some(Variant::Monochrome),
        1 => Some(Variant::Neutral),
        2 => Some(Variant::TonalSpot),
        3 => Some(Variant::Vibrant),
        4 => Some(Variant::Expressive),
        5 => Some(Variant::Fidelity),
        6 => Some(Variant::Content),
        7 => Some(Variant::Rainbow),
        8 => Some(Variant::FruitSalad),
        _ => None,
    }
}

fn role_from_i32(value: i32) -> Option<DynamicColorRole> {
    usize::try_from(value)
        .ok()
        .and_then(|index| DynamicColorRole::ALL.get(index).copied())
}

#[allow(clippy::cast_lossless)]
const fn parse_version_component(value: &str) -> u32 {
    let bytes = value.as_bytes();
    let mut index = 0;
    let mut parsed = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        assert!(byte >= b'0' && byte <= b'9');
        parsed = parsed * 10 + (byte - b'0') as u32;
        index += 1;
    }

    parsed
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_version_major() -> u32 {
    VERSION_MAJOR
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_version_minor() -> u32 {
    VERSION_MINOR
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_version_patch() -> u32 {
    VERSION_PATCH
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
    Argb::from_rgb(red, green, blue).to_u32()
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_lstar_from_argb(argb: u32) -> f64 {
    utils::lstar_from_argb(Argb::new(argb))
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_int_from_lstar(lstar: f64) -> u32 {
    utils::int_from_lstar(lstar).to_u32()
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_hct_from_argb(
    argb: u32,
    out_hct: *mut MaterialColorHct,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let hct = Hct::from_argb(Argb::new(argb));
        write_out(
            out_hct,
            MaterialColorHct {
                hue: hct.hue(),
                chroma: hct.chroma(),
                tone: hct.tone(),
                argb: hct.to_argb().to_u32(),
            },
        )
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_hct_to_argb(
    hue: f64,
    chroma: f64,
    tone: f64,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| write_out(out_argb, Hct::new(hue, chroma, tone).to_argb().to_u32()))
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_tonal_palette_get(
    hue: f64,
    chroma: f64,
    tone: f64,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let palette = TonalPalette::from_hue_and_chroma(hue, chroma);
        write_out(out_argb, palette.get(tone).to_u32())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_blend_harmonize(
    design_color: u32,
    key_color: u32,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        write_out(
            out_argb,
            blend::harmonize(Argb::new(design_color), Argb::new(key_color)).to_u32(),
        )
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_blend_hct_hue(
    from: u32,
    to: u32,
    amount: f64,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        write_out(
            out_argb,
            blend::hct_hue(Argb::new(from), Argb::new(to), amount).to_u32(),
        )
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_blend_cam16_ucs(
    from: u32,
    to: u32,
    amount: f64,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        write_out(
            out_argb,
            blend::cam16_ucs(Argb::new(from), Argb::new(to), amount).to_u32(),
        )
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_contrast_ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
    contrast::ratio_of_tones(tone_a, tone_b)
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_dynamic_scheme_color(
    source_argb: u32,
    variant: i32,
    is_dark: bool,
    contrast_level: f64,
    role: i32,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let Some(variant) = variant_from_i32(variant) else {
            return MaterialColorStatus::InvalidArgument;
        };
        let Some(role) = role_from_i32(role) else {
            return MaterialColorStatus::InvalidArgument;
        };
        let scheme =
            DynamicScheme::from_argb(Argb::new(source_argb), variant, is_dark, contrast_level);
        write_out(out_argb, scheme.color(role).to_u32())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_quantize_celebi(
    pixels: *const u32,
    pixel_count: usize,
    max_colors: u16,
    out_entries: *mut MaterialColorPopulationArray,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let Some(pixels) = slice_from_raw(pixels, pixel_count) else {
            return MaterialColorStatus::InvalidArgument;
        };
        let pixels = pixels.iter().copied().map(Argb::new).collect::<Vec<_>>();
        let result = quantize::quantize_celebi(&pixels, max_colors);
        write_out(
            out_entries,
            population_array_from_map(result.color_to_count),
        )
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_score_ranked(
    entries: *const MaterialColorPopulation,
    entry_count: usize,
    desired: usize,
    fallback_color_argb: u32,
    filter: bool,
    out_colors: *mut MaterialColorArgbArray,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let Some(entries) = slice_from_raw(entries, entry_count) else {
            return MaterialColorStatus::InvalidArgument;
        };
        let mut populations = BTreeMap::<Argb, u32>::new();
        for entry in entries {
            *populations.entry(Argb::new(entry.argb)).or_default() += entry.population;
        }
        let colors = ranked_suggestions(
            &populations,
            ScoreOptions {
                desired,
                fallback_color_argb: Argb::new(fallback_color_argb),
                filter,
            },
        )
        .into_iter()
        .map(Argb::to_u32)
        .collect::<Vec<_>>();

        write_out(out_colors, argb_array_from_vec(colors))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_temperature_complement(
    argb: u32,
    out_argb: *mut u32,
) -> MaterialColorStatus {
    ffi_guard(|| {
        let mut cache = TemperatureCache::new(Hct::from_argb(Argb::new(argb)));
        write_out(out_argb, cache.complement().to_argb().to_u32())
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_temperature_analogous(
    argb: u32,
    count: usize,
    divisions: usize,
    out_colors: *mut MaterialColorArgbArray,
) -> MaterialColorStatus {
    ffi_guard(|| {
        if count == 0 || divisions == 0 {
            return MaterialColorStatus::InvalidArgument;
        }
        let mut cache = TemperatureCache::new(Hct::from_argb(Argb::new(argb)));
        let colors = cache
            .analogous_colors(count, divisions)
            .into_iter()
            .map(|hct| hct.to_argb().to_u32())
            .collect::<Vec<_>>();
        write_out(out_colors, argb_array_from_vec(colors))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_argb_array_free(array: MaterialColorArgbArray) {
    if !array.data.is_null() && array.len != 0 {
        unsafe {
            drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                array.data, array.len,
            )));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn material_color_population_array_free(array: MaterialColorPopulationArray) {
    if !array.data.is_null() && array.len != 0 {
        unsafe {
            drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                array.data, array.len,
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hct_ffi_smoke() {
        let mut hct = MaterialColorHct::default();
        assert_eq!(
            material_color_hct_from_argb(0xff00_00ff, &raw mut hct),
            MaterialColorStatus::Ok
        );
        assert!((hct.tone - 32.302_586).abs() < 0.001);

        let mut argb = 0;
        assert_eq!(
            material_color_hct_to_argb(hct.hue, hct.chroma, hct.tone, &raw mut argb),
            MaterialColorStatus::Ok
        );
        assert_eq!(argb, 0xff00_00ff);
    }

    #[test]
    fn dynamic_scheme_ffi_smoke() {
        let mut argb = 0;
        assert_eq!(
            material_color_dynamic_scheme_color(0xff00_00ff, 2, false, 0.0, 25, &raw mut argb),
            MaterialColorStatus::Ok
        );
        assert_ne!(argb, 0);
    }

    #[test]
    fn quantize_and_score_ffi_smoke() {
        let pixels = [
            0xffff_0000,
            0xffff_0000,
            0xff00_ff00,
            0xff00_ff00,
            0xff00_ff00,
        ];
        let mut entries = MaterialColorPopulationArray::default();
        assert_eq!(
            material_color_quantize_celebi(pixels.as_ptr(), pixels.len(), 256, &raw mut entries),
            MaterialColorStatus::Ok
        );
        assert_eq!(entries.len, 2);

        let mut colors = MaterialColorArgbArray::default();
        assert_eq!(
            material_color_score_ranked(
                entries.data,
                entries.len,
                2,
                0xff42_85f4,
                false,
                &raw mut colors
            ),
            MaterialColorStatus::Ok
        );
        assert!(!colors.data.is_null());
        assert!((1..=2).contains(&colors.len));

        material_color_argb_array_free(colors);
        material_color_population_array_free(entries);
    }

    #[test]
    fn null_output_is_invalid_argument() {
        assert_eq!(
            material_color_hct_from_argb(0xff00_00ff, ptr::null_mut()),
            MaterialColorStatus::InvalidArgument
        );
    }

    #[test]
    fn c_abi_version_matches_crate_metadata() {
        assert_eq!(material_color_version_major(), VERSION_MAJOR);
        assert_eq!(material_color_version_minor(), VERSION_MINOR);
        assert_eq!(material_color_version_patch(), VERSION_PATCH);
    }
}
