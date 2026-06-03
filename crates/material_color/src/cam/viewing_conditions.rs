#![allow(clippy::many_single_char_names)]

use crate::utils::{PI, WHITE_POINT_D65, lerp, y_from_lstar};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewingConditions {
    pub adapting_luminance: f64,
    pub background_lstar: f64,
    pub surround: f64,
    pub discounting_illuminant: bool,
    pub background_y_to_white_point_y: f64,
    pub aw: f64,
    pub nbb: f64,
    pub ncb: f64,
    pub c: f64,
    pub n_c: f64,
    pub fl: f64,
    pub fl_root: f64,
    pub z: f64,
    pub white_point: [f64; 3],
    pub rgb_d: [f64; 3],
}

impl ViewingConditions {
    pub const DEFAULT: Self = Self {
        adapting_luminance: 11.725_676_537,
        background_lstar: 50.0,
        surround: 2.0,
        discounting_illuminant: false,
        background_y_to_white_point_y: 0.184_186_503,
        aw: 29.981_000_9,
        nbb: 1.016_919_255,
        ncb: 1.016_919_255,
        c: 0.689_999_998,
        n_c: 1.0,
        fl: 0.388_481_468,
        fl_root: 0.789_482_653,
        z: 1.909_169_555,
        white_point: WHITE_POINT_D65,
        rgb_d: [1.021_177_769, 0.986_307_740, 0.933_960_497],
    };

    #[must_use]
    pub fn create(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> Self {
        let background_lstar = background_lstar.max(30.0);
        let rgb_w = [
            0.401_288 * white_point[0] + 0.650_173 * white_point[1] - 0.051_461 * white_point[2],
            -0.250_268 * white_point[0] + 1.204_414 * white_point[1] + 0.045_854 * white_point[2],
            -0.002_079 * white_point[0] + 0.048_952 * white_point[1] + 0.953_127 * white_point[2],
        ];
        let f = 0.8 + surround / 10.0;
        let c = if f >= 0.9 {
            lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };
        let d = if discounting_illuminant {
            1.0
        } else {
            f * (1.0 - (1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())
        }
        .clamp(0.0, 1.0);
        let rgb_d = [
            d * (100.0 / rgb_w[0]) + 1.0 - d,
            d * (100.0 / rgb_w[1]) + 1.0 - d,
            d * (100.0 / rgb_w[2]) + 1.0 - d,
        ];

        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl =
            k4 * adapting_luminance + 0.1 * k4f * k4f * (5.0 * adapting_luminance).powf(1.0 / 3.0);
        let fl_root = fl.powf(0.25);
        let n = y_from_lstar(background_lstar) / white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let rgb_a_factors = [
            (fl * rgb_d[0] * rgb_w[0] / 100.0).powf(0.42),
            (fl * rgb_d[1] * rgb_w[1] / 100.0).powf(0.42),
            (fl * rgb_d[2] * rgb_w[2] / 100.0).powf(0.42),
        ];
        let rgb_a = [
            400.0 * rgb_a_factors[0] / (rgb_a_factors[0] + 27.13),
            400.0 * rgb_a_factors[1] / (rgb_a_factors[1] + 27.13),
            400.0 * rgb_a_factors[2] / (rgb_a_factors[2] + 27.13),
        ];
        let aw = (40.0 * rgb_a[0] + 20.0 * rgb_a[1] + rgb_a[2]) / 20.0 * nbb;

        Self {
            adapting_luminance,
            background_lstar,
            surround,
            discounting_illuminant,
            background_y_to_white_point_y: n,
            aw,
            nbb,
            ncb: nbb,
            c,
            n_c: f,
            fl,
            fl_root,
            z,
            white_point,
            rgb_d,
        }
    }

    #[must_use]
    pub fn default_with_background_lstar(background_lstar: f64) -> Self {
        Self::create(
            WHITE_POINT_D65,
            200.0 / PI * y_from_lstar(50.0) / 100.0,
            background_lstar,
            2.0,
            false,
        )
    }
}

impl Default for ViewingConditions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[must_use]
pub fn create_viewing_conditions(
    white_point: [f64; 3],
    adapting_luminance: f64,
    background_lstar: f64,
    surround: f64,
    discounting_illuminant: bool,
) -> ViewingConditions {
    ViewingConditions::create(
        white_point,
        adapting_luminance,
        background_lstar,
        surround,
        discounting_illuminant,
    )
}

#[must_use]
pub fn default_with_background_lstar(background_lstar: f64) -> ViewingConditions {
    ViewingConditions::default_with_background_lstar(background_lstar)
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
    fn default_with_background_lstar_matches_cpp_default_frame() {
        let conditions = ViewingConditions::default_with_background_lstar(50.0);
        let default = ViewingConditions::DEFAULT;
        let tolerance = 1e-5;

        assert_near(
            conditions.adapting_luminance,
            default.adapting_luminance,
            tolerance,
        );
        assert_near(
            conditions.background_lstar,
            default.background_lstar,
            tolerance,
        );
        assert_near(conditions.surround, default.surround, tolerance);
        assert_eq!(
            conditions.discounting_illuminant,
            default.discounting_illuminant
        );
        assert_near(
            conditions.background_y_to_white_point_y,
            default.background_y_to_white_point_y,
            tolerance,
        );
        assert_near(conditions.aw, default.aw, tolerance);
        assert_near(conditions.nbb, default.nbb, tolerance);
        assert_near(conditions.ncb, default.ncb, tolerance);
        assert_near(conditions.c, default.c, tolerance);
        assert_near(conditions.n_c, default.n_c, tolerance);
        assert_near(conditions.fl, default.fl, tolerance);
        assert_near(conditions.fl_root, default.fl_root, tolerance);
        assert_near(conditions.z, default.z, tolerance);

        for index in 0..3 {
            assert_near(
                conditions.white_point[index],
                default.white_point[index],
                tolerance,
            );
            assert_near(conditions.rgb_d[index], default.rgb_d[index], tolerance);
        }
    }

    #[test]
    fn low_background_lstar_is_corrected_to_thirty() {
        assert_eq!(
            ViewingConditions::default_with_background_lstar(10.0).background_lstar,
            30.0
        );
    }
}
