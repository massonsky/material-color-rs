use crate::utils::{lstar_from_y, y_from_lstar};

const CONTRAST_RATIO_EPSILON: f64 = 0.04;
const LUMINANCE_GAMUT_MAP_TOLERANCE: f64 = 0.4;

#[must_use]
pub fn ratio_of_ys(y1: f64, y2: f64) -> f64 {
    let lighter = y1.max(y2);
    let darker = y1.min(y2);

    (lighter + 5.0) / (darker + 5.0)
}

#[must_use]
pub fn ratio_of_tones(tone_a: f64, tone_b: f64) -> f64 {
    ratio_of_ys(
        y_from_lstar(tone_a.clamp(0.0, 100.0)),
        y_from_lstar(tone_b.clamp(0.0, 100.0)),
    )
}

#[must_use]
pub fn lighter(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }

    let dark_y = y_from_lstar(tone);
    let light_y = ratio * (dark_y + 5.0) - 5.0;
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
        return -1.0;
    }

    let value = lstar_from_y(light_y) + LUMINANCE_GAMUT_MAP_TOLERANCE;
    if !(0.0..=100.0).contains(&value) {
        return -1.0;
    }

    value
}

#[must_use]
pub fn darker(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }

    let light_y = y_from_lstar(tone);
    let dark_y = ((light_y + 5.0) / ratio) - 5.0;
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
        return -1.0;
    }

    let value = lstar_from_y(dark_y) - LUMINANCE_GAMUT_MAP_TOLERANCE;
    if !(0.0..=100.0).contains(&value) {
        return -1.0;
    }

    value
}

#[must_use]
pub fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
    let safe = lighter(tone, ratio);
    if safe < 0.0 { 100.0 } else { safe }
}

#[must_use]
pub fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
    let safe = darker(tone, ratio);
    if safe < 0.0 { 0.0 } else { safe }
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
    fn ratio_of_tones_clamps_out_of_bounds_input() {
        assert_near(ratio_of_tones(-10.0, 110.0), 21.0, 0.001);
    }

    #[test]
    fn lighter_impossible_ratio_errors() {
        assert_near(lighter(90.0, 10.0), -1.0, 0.001);
    }

    #[test]
    fn lighter_out_of_bounds_input_errors() {
        assert_near(lighter(110.0, 2.0), -1.0, 0.001);
        assert_near(lighter(-10.0, 2.0), -1.0, 0.001);
    }

    #[test]
    fn lighter_unsafe_returns_max_tone() {
        assert_near(lighter_unsafe(100.0, 2.0), 100.0, 0.001);
    }

    #[test]
    fn darker_impossible_ratio_errors() {
        assert_near(darker(10.0, 20.0), -1.0, 0.001);
    }

    #[test]
    fn darker_out_of_bounds_input_errors() {
        assert_near(darker(110.0, 2.0), -1.0, 0.001);
        assert_near(darker(-10.0, 2.0), -1.0, 0.001);
    }

    #[test]
    fn darker_unsafe_returns_min_tone() {
        assert_near(darker_unsafe(0.0, 2.0), 0.0, 0.001);
    }
}
