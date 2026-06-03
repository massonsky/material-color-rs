use crate::argb::Argb;
use crate::cam::Hct;
use crate::palettes::tones::TonalPalette;
use crate::utils::sanitize_degrees_double;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorePalettes {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
}

impl CorePalettes {
    #[must_use]
    pub fn from_source_color(source: Argb) -> Self {
        Self::tonal_spot(Hct::from_argb(source))
    }

    #[must_use]
    pub fn tonal_spot(source: Hct) -> Self {
        let hue = source.hue();

        Self {
            primary: TonalPalette::from_hue_and_chroma(hue, 36.0),
            secondary: TonalPalette::from_hue_and_chroma(hue, 16.0),
            tertiary: TonalPalette::from_hue_and_chroma(sanitize_degrees_double(hue + 60.0), 24.0),
            neutral: TonalPalette::from_hue_and_chroma(hue, 6.0),
            neutral_variant: TonalPalette::from_hue_and_chroma(hue, 8.0),
        }
    }

    #[must_use]
    pub fn vibrant(source: Hct) -> Self {
        const HUES: [f64; 9] = [0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];
        const SECONDARY_ROTATIONS: [f64; 9] =
            [18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];
        const TERTIARY_ROTATIONS: [f64; 9] = [35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];

        Self {
            primary: TonalPalette::from_hue_and_chroma(source.hue(), 200.0),
            secondary: TonalPalette::from_hue_and_chroma(
                rotated_hue(source, &HUES, &SECONDARY_ROTATIONS),
                24.0,
            ),
            tertiary: TonalPalette::from_hue_and_chroma(
                rotated_hue(source, &HUES, &TERTIARY_ROTATIONS),
                32.0,
            ),
            neutral: TonalPalette::from_hue_and_chroma(source.hue(), 10.0),
            neutral_variant: TonalPalette::from_hue_and_chroma(source.hue(), 12.0),
        }
    }

    #[must_use]
    pub fn expressive(source: Hct) -> Self {
        const HUES: [f64; 9] = [0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0];
        const SECONDARY_ROTATIONS: [f64; 9] =
            [45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0];
        const TERTIARY_ROTATIONS: [f64; 9] =
            [120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0];

        Self {
            primary: TonalPalette::from_hue_and_chroma(source.hue() + 240.0, 40.0),
            secondary: TonalPalette::from_hue_and_chroma(
                rotated_hue(source, &HUES, &SECONDARY_ROTATIONS),
                24.0,
            ),
            tertiary: TonalPalette::from_hue_and_chroma(
                rotated_hue(source, &HUES, &TERTIARY_ROTATIONS),
                32.0,
            ),
            neutral: TonalPalette::from_hue_and_chroma(source.hue() + 15.0, 8.0),
            neutral_variant: TonalPalette::from_hue_and_chroma(source.hue() + 15.0, 12.0),
        }
    }

    #[must_use]
    pub fn neutral(source: Hct) -> Self {
        let hue = source.hue();

        Self {
            primary: TonalPalette::from_hue_and_chroma(hue, 12.0),
            secondary: TonalPalette::from_hue_and_chroma(hue, 8.0),
            tertiary: TonalPalette::from_hue_and_chroma(hue, 16.0),
            neutral: TonalPalette::from_hue_and_chroma(hue, 2.0),
            neutral_variant: TonalPalette::from_hue_and_chroma(hue, 2.0),
        }
    }

    #[must_use]
    pub fn monochrome(source: Hct) -> Self {
        let hue = source.hue();

        Self {
            primary: TonalPalette::from_hue_and_chroma(hue, 0.0),
            secondary: TonalPalette::from_hue_and_chroma(hue, 0.0),
            tertiary: TonalPalette::from_hue_and_chroma(hue, 0.0),
            neutral: TonalPalette::from_hue_and_chroma(hue, 0.0),
            neutral_variant: TonalPalette::from_hue_and_chroma(hue, 0.0),
        }
    }
}

#[must_use]
pub fn rotated_hue(source: Hct, hues: &[f64], rotations: &[f64]) -> f64 {
    let source_hue = source.hue();

    if rotations.len() == 1 {
        return sanitize_degrees_double(source_hue + rotations[0]);
    }

    for (index, window) in hues.windows(2).enumerate() {
        if window[0] < source_hue && source_hue < window[1] {
            return sanitize_degrees_double(source_hue + rotations[index]);
        }
    }

    source_hue
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    #[test]
    fn tonal_spot_core_palettes_match_cpp_recipe() {
        let source = Hct::from_argb(Argb::new(0xff00_00ff));
        let palettes = CorePalettes::tonal_spot(source);

        assert_near(palettes.primary.hue(), source.hue(), 0.001);
        assert_eq!(palettes.primary.chroma(), 36.0);
        assert_eq!(palettes.secondary.chroma(), 16.0);
        assert_near(
            palettes.tertiary.hue(),
            sanitize_degrees_double(source.hue() + 60.0),
            0.001,
        );
        assert_eq!(palettes.tertiary.chroma(), 24.0);
        assert_eq!(palettes.neutral.chroma(), 6.0);
        assert_eq!(palettes.neutral_variant.chroma(), 8.0);
    }

    #[test]
    fn vibrant_core_palettes_rotate_blue_like_cpp_recipe() {
        let source = Hct::from_argb(Argb::new(0xff00_00ff));
        let palettes = CorePalettes::vibrant(source);

        assert_eq!(palettes.primary.chroma(), 200.0);
        assert_near(palettes.secondary.hue(), source.hue() + 15.0, 0.001);
        assert_eq!(palettes.secondary.chroma(), 24.0);
        assert_near(palettes.tertiary.hue(), source.hue() + 30.0, 0.001);
        assert_eq!(palettes.tertiary.chroma(), 32.0);
        assert_eq!(palettes.neutral.chroma(), 10.0);
        assert_eq!(palettes.neutral_variant.chroma(), 12.0);
    }

    #[test]
    fn expressive_core_palettes_rotate_blue_like_cpp_recipe() {
        let source = Hct::from_argb(Argb::new(0xff00_00ff));
        let palettes = CorePalettes::expressive(source);

        assert_near(palettes.primary.hue(), source.hue() + 240.0, 0.001);
        assert_eq!(palettes.primary.chroma(), 40.0);
        assert_near(palettes.secondary.hue(), source.hue() + 45.0, 0.001);
        assert_eq!(palettes.secondary.chroma(), 24.0);
        assert_near(palettes.tertiary.hue(), source.hue() + 20.0, 0.001);
        assert_eq!(palettes.tertiary.chroma(), 32.0);
        assert_near(palettes.neutral.hue(), source.hue() + 15.0, 0.001);
        assert_eq!(palettes.neutral.chroma(), 8.0);
        assert_eq!(palettes.neutral_variant.chroma(), 12.0);
    }

    #[test]
    fn neutral_and_monochrome_core_palettes_match_cpp_recipe() {
        let source = Hct::from_argb(Argb::new(0xff00_00ff));
        let neutral = CorePalettes::neutral(source);
        let monochrome = CorePalettes::monochrome(source);

        assert_eq!(neutral.primary.chroma(), 12.0);
        assert_eq!(neutral.secondary.chroma(), 8.0);
        assert_eq!(neutral.tertiary.chroma(), 16.0);
        assert_eq!(neutral.neutral.chroma(), 2.0);
        assert_eq!(neutral.neutral_variant.chroma(), 2.0);
        assert_eq!(monochrome.primary.chroma(), 0.0);
        assert_eq!(monochrome.secondary.chroma(), 0.0);
        assert_eq!(monochrome.tertiary.chroma(), 0.0);
        assert_eq!(monochrome.neutral.chroma(), 0.0);
        assert_eq!(monochrome.neutral_variant.chroma(), 0.0);
    }

    #[test]
    fn rotated_hue_matches_dynamic_scheme_boundaries() {
        let source = Hct::new(30.0, 40.0, 50.0);
        let source_hue = source.hue();

        assert_eq!(
            rotated_hue(source, &[0.0, 40.0, 360.0], &[10.0, 20.0, 20.0]),
            sanitize_degrees_double(source_hue + 10.0)
        );
        assert_eq!(
            rotated_hue(source, &[0.0, 20.0, 360.0], &[10.0, 20.0, 20.0]),
            sanitize_degrees_double(source_hue + 20.0)
        );
        assert_eq!(
            rotated_hue(source, &[0.0, 20.0, 360.0], &[15.0]),
            sanitize_degrees_double(source_hue + 15.0)
        );
    }
}
