use std::collections::HashMap;

use crate::argb::Argb;
use crate::cam::{Cam, Hct};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TonalPalette {
    hue: f64,
    chroma: f64,
    key_color: Hct,
}

impl TonalPalette {
    #[must_use]
    pub fn from_argb(argb: Argb) -> Self {
        let cam = Cam::from_argb(argb);

        Self {
            hue: cam.hue,
            chroma: cam.chroma,
            key_color: KeyColor::new(cam.hue, cam.chroma).create(),
        }
    }

    #[must_use]
    pub fn from_hct(hct: Hct) -> Self {
        Self {
            hue: hct.hue(),
            chroma: hct.chroma(),
            key_color: hct,
        }
    }

    #[must_use]
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
        Self {
            hue,
            chroma,
            key_color: KeyColor::new(hue, chroma).create(),
        }
    }

    #[must_use]
    pub const fn from_hue_chroma_and_key_color(hue: f64, chroma: f64, key_color: Hct) -> Self {
        Self {
            hue,
            chroma,
            key_color,
        }
    }

    #[must_use]
    pub const fn hue(self) -> f64 {
        self.hue
    }

    #[must_use]
    pub const fn chroma(self) -> f64 {
        self.chroma
    }

    #[must_use]
    pub const fn key_color(self) -> Hct {
        self.key_color
    }

    #[must_use]
    pub fn get(self, tone: f64) -> Argb {
        Hct::new(self.hue, self.chroma, tone).to_argb()
    }
}

#[derive(Debug)]
struct KeyColor {
    hue: f64,
    requested_chroma: f64,
    chroma_cache: HashMap<i32, f64>,
}

impl KeyColor {
    const MAX_CHROMA: f64 = 200.0;

    fn new(hue: f64, requested_chroma: f64) -> Self {
        Self {
            hue,
            requested_chroma,
            chroma_cache: HashMap::new(),
        }
    }

    fn create(mut self) -> Hct {
        let pivot_tone = 50;
        let tone_step_size = 1;
        let epsilon = 0.01;
        let mut lower_tone = 0;
        let mut upper_tone = 100;

        while lower_tone < upper_tone {
            let mid_tone = (lower_tone + upper_tone) / 2;
            let is_ascending =
                self.max_chroma(mid_tone) < self.max_chroma(mid_tone + tone_step_size);
            let sufficient_chroma = self.max_chroma(mid_tone) >= self.requested_chroma - epsilon;

            if sufficient_chroma {
                if (lower_tone - pivot_tone).abs() < (upper_tone - pivot_tone).abs() {
                    upper_tone = mid_tone;
                } else if lower_tone == mid_tone {
                    return Hct::new(self.hue, self.requested_chroma, f64::from(lower_tone));
                } else {
                    lower_tone = mid_tone;
                }
            } else if is_ascending {
                lower_tone = mid_tone + tone_step_size;
            } else {
                upper_tone = mid_tone;
            }
        }

        Hct::new(self.hue, self.requested_chroma, f64::from(lower_tone))
    }

    fn max_chroma(&mut self, tone: i32) -> f64 {
        if let Some(chroma) = self.chroma_cache.get(&tone) {
            return *chroma;
        }

        let chroma = Hct::new(self.hue, Self::MAX_CHROMA, f64::from(tone)).chroma();
        self.chroma_cache.insert(tone, chroma);

        chroma
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hex_from_argb;

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    #[test]
    fn blue_palette_matches_cpp_golden_values() {
        let palette = TonalPalette::from_argb(Argb::new(0xff00_00ff));

        for (tone, expected) in [
            (100.0, "ffffffff"),
            (95.0, "fff1efff"),
            (90.0, "ffe0e0ff"),
            (80.0, "ffbec2ff"),
            (70.0, "ff9da3ff"),
            (60.0, "ff7c84ff"),
            (50.0, "ff5a64ff"),
            (40.0, "ff343dff"),
            (30.0, "ff0000ef"),
            (20.0, "ff0001ac"),
            (10.0, "ff00006e"),
            (0.0, "ff000000"),
        ] {
            assert_eq!(hex_from_argb(palette.get(tone)), expected);
        }
    }

    #[test]
    fn key_color_exact_chroma_available_matches_cpp() {
        let palette = TonalPalette::from_hue_and_chroma(50.0, 60.0);
        let key_color = palette.key_color();

        assert_near(key_color.hue(), 50.0, 10.0);
        assert_near(key_color.chroma(), 60.0, 0.5);
        assert!(key_color.tone() > 0.0);
        assert!(key_color.tone() < 100.0);
    }

    #[test]
    fn key_color_unusually_high_chroma_matches_cpp() {
        let palette = TonalPalette::from_hue_and_chroma(149.0, 200.0);
        let key_color = palette.key_color();

        assert_near(key_color.hue(), 149.0, 10.0);
        assert!(key_color.chroma() > 89.0);
        assert!(key_color.tone() > 0.0);
        assert!(key_color.tone() < 100.0);
    }

    #[test]
    fn key_color_unusually_low_chroma_matches_cpp() {
        let palette = TonalPalette::from_hue_and_chroma(50.0, 3.0);
        let key_color = palette.key_color();

        assert_near(key_color.hue(), 50.0, 10.0);
        assert_near(key_color.chroma(), 3.0, 0.5);
        assert_near(key_color.tone(), 50.0, 0.5);
    }
}
