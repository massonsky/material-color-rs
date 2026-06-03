use std::collections::BTreeMap;

use material_color::TemperatureCache;
use material_color::VERSION;
use material_color::argb::Argb;
use material_color::blend;
use material_color::cam::Hct;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};
use material_color::palettes::TonalPalette;
use material_color::quantize;
use material_color::score::{ScoreOptions, ranked_suggestions};

fn assert_near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual {actual} expected {expected} tolerance {tolerance}",
    );
}

#[test]
fn rust_version_surface_is_stable() {
    assert_eq!(VERSION, "1.0.0");
}

#[test]
fn source_color_workflow_snapshots_match_cpp_reference_values() {
    let blue = Argb::new(0xff42_85f4);
    let hct = Hct::from_argb(blue);
    assert_near(hct.hue(), 265.979_403, 0.001);
    assert_near(hct.chroma(), 62.269_111, 0.001);
    assert_near(hct.tone(), 56.550_349, 0.001);

    let palette = TonalPalette::from_argb(blue);
    assert_eq!(palette.key_color().to_argb(), Argb::new(0xff2b_74e2));
    assert_eq!(palette.get(40.0), Argb::new(0xff00_5ac1));

    let light = DynamicScheme::from_argb(blue, Variant::TonalSpot, false, 0.0);
    assert_eq!(
        light.color(DynamicColorRole::Primary),
        Argb::new(0xff44_5e91)
    );
    assert_eq!(
        light.color(DynamicColorRole::OnPrimary),
        Argb::new(0xffff_ffff),
    );
    assert_eq!(
        light.color(DynamicColorRole::PrimaryContainer),
        Argb::new(0xffd8_e2ff),
    );
    assert_eq!(
        light.color(DynamicColorRole::Surface),
        Argb::new(0xfff9_f9ff)
    );
    assert_eq!(
        light.color(DynamicColorRole::OnSurface),
        Argb::new(0xff1a_1b20),
    );

    let dark = DynamicScheme::from_argb(blue, Variant::TonalSpot, true, 0.0);
    assert_eq!(
        dark.color(DynamicColorRole::Primary),
        Argb::new(0xffad_c6ff)
    );
    assert_eq!(
        dark.color(DynamicColorRole::OnPrimary),
        Argb::new(0xff10_2f60),
    );
    assert_eq!(
        dark.color(DynamicColorRole::Surface),
        Argb::new(0xff11_1318)
    );
    assert_eq!(
        dark.color(DynamicColorRole::OnSurface),
        Argb::new(0xffe2_e2e9),
    );
}

#[test]
fn representative_variant_snapshots_match_cpp_reference_values() {
    let cases = [
        (
            0xffff_0000,
            Variant::Vibrant,
            0xffc0_0100,
            0xff80_543d,
            0xff83_5423,
        ),
        (
            0xff67_50a4,
            Variant::Expressive,
            0xff00_6b5a,
            0xff79_536a,
            0xff73_5280,
        ),
        (
            0xff00_6e1c,
            Variant::TonalSpot,
            0xff3b_6939,
            0xff52_634f,
            0xff38_656a,
        ),
    ];

    for (source, variant, primary, secondary, tertiary) in cases {
        let scheme = DynamicScheme::from_argb(Argb::new(source), variant, false, 0.0);
        assert_eq!(scheme.color(DynamicColorRole::Primary), Argb::new(primary));
        assert_eq!(
            scheme.color(DynamicColorRole::Secondary),
            Argb::new(secondary),
        );
        assert_eq!(
            scheme.color(DynamicColorRole::Tertiary),
            Argb::new(tertiary)
        );
        assert_eq!(
            scheme.color(DynamicColorRole::Error),
            Argb::new(0xffba_1a1a)
        );
    }
}

#[test]
fn extraction_scoring_blending_and_temperature_snapshots_match_cpp_reference_values() {
    let pixels = [
        0xffff_0000,
        0xffff_0000,
        0xffff_0000,
        0xff00_ff00,
        0xff00_ff00,
        0xff00_00ff,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0xff00_0000,
        0xff00_0000,
    ]
    .into_iter()
    .map(Argb::new)
    .collect::<Vec<_>>();

    let quantized = quantize::quantize_celebi(&pixels, 4);
    let expected_populations = BTreeMap::from([
        (Argb::new(0xff00_00ff), 1),
        (Argb::new(0xff24_7719), 4),
        (Argb::new(0xffff_0000), 3),
        (Argb::new(0xffff_ffff), 4),
    ]);
    assert_eq!(quantized.color_to_count, expected_populations);

    let suggestions = ranked_suggestions(
        &quantized.color_to_count,
        ScoreOptions {
            desired: 4,
            fallback_color_argb: Argb::new(0xff42_85f4),
            filter: true,
        },
    );
    assert_eq!(
        suggestions,
        [
            Argb::new(0xffff_0000),
            Argb::new(0xff24_7719),
            Argb::new(0xff00_00ff),
        ],
    );

    let blue = Argb::new(0xff42_85f4);
    assert_eq!(
        blend::harmonize(Argb::new(0xffff_0000), blue),
        Argb::new(0xfffb_0057),
    );
    assert_eq!(
        blend::hct_hue(Argb::new(0xffff_0000), blue, 0.5),
        Argb::new(0xffeb_00ba),
    );

    let mut temperature = TemperatureCache::new(Hct::from_argb(blue));
    assert_eq!(temperature.complement().to_argb(), Argb::new(0xffd0_6e00));
    assert_eq!(
        temperature
            .analogous_colors(5, 12)
            .into_iter()
            .map(Hct::to_argb)
            .collect::<Vec<_>>(),
        [
            Argb::new(0xff1d_9c46),
            Argb::new(0xff00_988c),
            Argb::new(0xff42_85f4),
            Argb::new(0xff96_71ed),
            Argb::new(0xffbc_64d0),
        ],
    );
}
