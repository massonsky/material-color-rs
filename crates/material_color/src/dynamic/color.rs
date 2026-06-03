use crate::contrast::{darker, darker_unsafe, lighter, lighter_unsafe, ratio_of_tones};
use crate::utils::lerp;

use super::roles::DynamicColorRole;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContrastCurve {
    low: f64,
    normal: f64,
    medium: f64,
    high: f64,
}

impl ContrastCurve {
    #[must_use]
    pub const fn new(low: f64, normal: f64, medium: f64, high: f64) -> Self {
        Self {
            low,
            normal,
            medium,
            high,
        }
    }

    #[must_use]
    pub fn get(self, contrast_level: f64) -> f64 {
        if contrast_level <= -1.0 {
            self.low
        } else if contrast_level < 0.0 {
            lerp(self.low, self.normal, contrast_level + 1.0)
        } else if contrast_level < 0.5 {
            lerp(self.normal, self.medium, contrast_level / 0.5)
        } else if contrast_level < 1.0 {
            lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5)
        } else {
            self.high
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TonePolarity {
    Darker,
    Lighter,
    Nearer,
    Farther,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToneDeltaPair {
    pub role_a: DynamicColorRole,
    pub role_b: DynamicColorRole,
    pub delta: f64,
    pub polarity: TonePolarity,
    pub stay_together: bool,
}

impl ToneDeltaPair {
    #[must_use]
    pub const fn new(
        role_a: DynamicColorRole,
        role_b: DynamicColorRole,
        delta: f64,
        polarity: TonePolarity,
        stay_together: bool,
    ) -> Self {
        Self {
            role_a,
            role_b,
            delta,
            polarity,
            stay_together,
        }
    }
}

#[must_use]
pub fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
    let lighter_tone = lighter_unsafe(bg_tone, ratio);
    let darker_tone = darker_unsafe(bg_tone, ratio);
    let lighter_ratio = ratio_of_tones(lighter_tone, bg_tone);
    let darker_ratio = ratio_of_tones(darker_tone, bg_tone);

    if tone_prefers_light_foreground(bg_tone) {
        let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1
            && lighter_ratio < ratio
            && darker_ratio < ratio;

        if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference {
            lighter_tone
        } else {
            darker_tone
        }
    } else if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
        darker_tone
    } else {
        lighter_tone
    }
}

#[must_use]
pub fn enable_light_foreground(tone: f64) -> f64 {
    if tone_prefers_light_foreground(tone) && !tone_allows_light_foreground(tone) {
        49.0
    } else {
        tone
    }
}

#[must_use]
pub fn tone_prefers_light_foreground(tone: f64) -> bool {
    tone.round() < 60.0
}

#[must_use]
pub fn tone_allows_light_foreground(tone: f64) -> bool {
    tone.round() <= 49.0
}

#[must_use]
pub(crate) fn dual_background_tone(
    answer: f64,
    bg_tone_1: f64,
    bg_tone_2: f64,
    desired_ratio: f64,
) -> f64 {
    let upper = bg_tone_1.max(bg_tone_2);
    let lower = bg_tone_1.min(bg_tone_2);

    if ratio_of_tones(upper, answer) >= desired_ratio
        && ratio_of_tones(lower, answer) >= desired_ratio
    {
        return answer;
    }

    let light_option = lighter(upper, desired_ratio);
    let dark_option = darker(lower, desired_ratio);
    let prefers_light =
        tone_prefers_light_foreground(bg_tone_1) || tone_prefers_light_foreground(bg_tone_2);

    if prefers_light {
        if light_option < 0.0 {
            100.0
        } else {
            light_option
        }
    } else if light_option >= 0.0 && dark_option < 0.0 {
        light_option
    } else if dark_option < 0.0 {
        0.0
    } else {
        dark_option
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    #[test]
    fn contrast_curve_interpolates_cpp_breakpoints() {
        let curve = ContrastCurve::new(1.0, 3.0, 7.0, 11.0);

        assert_near(curve.get(-2.0), 1.0, 0.001);
        assert_near(curve.get(-0.5), 2.0, 0.001);
        assert_near(curve.get(0.25), 5.0, 0.001);
        assert_near(curve.get(0.75), 9.0, 0.001);
        assert_near(curve.get(2.0), 11.0, 0.001);
    }

    #[test]
    fn foreground_preference_matches_cpp_thresholds() {
        assert!(tone_prefers_light_foreground(59.0));
        assert!(!tone_prefers_light_foreground(60.0));
        assert!(tone_allows_light_foreground(49.0));
        assert!(!tone_allows_light_foreground(50.0));
        assert_near(enable_light_foreground(55.0), 49.0, 0.001);
        assert_near(enable_light_foreground(45.0), 45.0, 0.001);
    }
}
