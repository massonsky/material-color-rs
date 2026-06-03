use crate::argb::{Argb, argb_from_rgb};

pub const PI: f64 = std::f64::consts::PI;
pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Vec3 {
    #[must_use]
    pub const fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

#[must_use]
pub fn argb_from_linrgb(linrgb: Vec3) -> Argb {
    let red = delinearized(linrgb.a);
    let green = delinearized(linrgb.b);
    let blue = delinearized(linrgb.c);

    argb_from_rgb(red, green, blue)
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn delinearized(rgb_component: f64) -> i32 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.003_130_8 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };

    (delinearized * 255.0).round().clamp(0.0, 255.0) as i32
}

#[must_use]
pub fn linearized(rgb_component: u8) -> f64 {
    let normalized = f64::from(rgb_component) / 255.0;
    if normalized <= 0.040_449_936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

#[must_use]
pub fn lstar_from_argb(argb: Argb) -> f64 {
    let red_l = linearized(argb.red());
    let green_l = linearized(argb.green());
    let blue_l = linearized(argb.blue());
    let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;

    lstar_from_y(y)
}

#[must_use]
pub fn y_from_lstar(lstar: f64) -> f64 {
    if lstar > 8.0 {
        let cube_root = (lstar + 16.0) / 116.0;
        cube_root * cube_root * cube_root * 100.0
    } else {
        lstar / (24_389.0 / 27.0) * 100.0
    }
}

#[must_use]
pub fn lstar_from_y(y: f64) -> f64 {
    let y_normalized = y / 100.0;
    if y_normalized <= 216.0 / 24_389.0 {
        (24_389.0 / 27.0) * y_normalized
    } else {
        116.0 * y_normalized.powf(1.0 / 3.0) - 16.0
    }
}

#[must_use]
pub fn sanitize_degrees_int(degrees: i32) -> i32 {
    if degrees < 0 {
        (degrees % 360) + 360
    } else if degrees >= 360 {
        degrees % 360
    } else {
        degrees
    }
}

#[must_use]
pub fn sanitize_degrees_double(degrees: f64) -> f64 {
    if degrees < 0.0 {
        (degrees % 360.0) + 360.0
    } else if degrees >= 360.0 {
        degrees % 360.0
    } else {
        degrees
    }
}

#[must_use]
pub fn diff_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

#[must_use]
pub fn rotation_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);
    if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

#[must_use]
pub fn hex_from_argb(argb: Argb) -> String {
    format!("{:x}", argb.to_u32())
}

#[must_use]
pub fn int_from_lstar(lstar: f64) -> Argb {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);

    argb_from_rgb(component, component, component)
}

#[must_use]
#[allow(clippy::bool_to_int_with_if, clippy::float_cmp)]
pub fn signum(num: f64) -> i32 {
    if num < 0.0 {
        -1
    } else if num == 0.0 {
        0
    } else {
        1
    }
}

#[must_use]
pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

#[must_use]
pub fn matrix_multiply(input: Vec3, matrix: &[[f64; 3]; 3]) -> Vec3 {
    Vec3 {
        a: input.a * matrix[0][0] + input.b * matrix[0][1] + input.c * matrix[0][2],
        b: input.a * matrix[1][0] + input.b * matrix[1][1] + input.c * matrix[1][2],
        c: input.a * matrix[2][0] + input.b * matrix[2][1] + input.c * matrix[2][2],
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    const MATRIX: [[f64; 3]; 3] = [[1.0, 2.0, 3.0], [-4.0, 5.0, -6.0], [-7.0, -8.0, -9.0]];

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    #[test]
    fn signum_matches_cpp() {
        assert_eq!(signum(0.001), 1);
        assert_eq!(signum(3.0), 1);
        assert_eq!(signum(100.0), 1);
        assert_eq!(signum(-0.002), -1);
        assert_eq!(signum(-4.0), -1);
        assert_eq!(signum(-101.0), -1);
        assert_eq!(signum(0.0), 0);
    }

    #[test]
    fn rotation_is_positive_for_counterclockwise() {
        assert_eq!(rotation_direction(0.0, 30.0), 1.0);
        assert_eq!(rotation_direction(0.0, 60.0), 1.0);
        assert_eq!(rotation_direction(0.0, 150.0), 1.0);
        assert_eq!(rotation_direction(90.0, 240.0), 1.0);
        assert_eq!(rotation_direction(300.0, 30.0), 1.0);
        assert_eq!(rotation_direction(270.0, 60.0), 1.0);
        assert_eq!(rotation_direction(360.0 * 2.0, 15.0), 1.0);
        assert_eq!(
            rotation_direction(360.0 * 3.0 + 15.0, -360.0 * 4.0 + 30.0),
            1.0
        );
    }

    #[test]
    fn rotation_is_negative_for_clockwise() {
        assert_eq!(rotation_direction(30.0, 0.0), -1.0);
        assert_eq!(rotation_direction(60.0, 0.0), -1.0);
        assert_eq!(rotation_direction(150.0, 0.0), -1.0);
        assert_eq!(rotation_direction(240.0, 90.0), -1.0);
        assert_eq!(rotation_direction(30.0, 300.0), -1.0);
        assert_eq!(rotation_direction(60.0, 270.0), -1.0);
        assert_eq!(rotation_direction(15.0, -360.0 * 2.0), -1.0);
        assert_eq!(
            rotation_direction(-360.0 * 4.0 + 270.0, 360.0 * 5.0 + 180.0),
            -1.0
        );
    }

    #[test]
    fn angle_difference_matches_cpp() {
        assert_eq!(diff_degrees(0.0, 30.0), 30.0);
        assert_eq!(diff_degrees(0.0, 60.0), 60.0);
        assert_eq!(diff_degrees(0.0, 150.0), 150.0);
        assert_eq!(diff_degrees(90.0, 240.0), 150.0);
        assert_eq!(diff_degrees(300.0, 30.0), 90.0);
        assert_eq!(diff_degrees(270.0, 60.0), 150.0);
        assert_eq!(diff_degrees(30.0, 0.0), 30.0);
        assert_eq!(diff_degrees(60.0, 0.0), 60.0);
        assert_eq!(diff_degrees(150.0, 0.0), 150.0);
        assert_eq!(diff_degrees(240.0, 90.0), 150.0);
        assert_eq!(diff_degrees(30.0, 300.0), 90.0);
        assert_eq!(diff_degrees(60.0, 270.0), 150.0);
    }

    #[test]
    fn angle_sanitation_matches_cpp() {
        assert_eq!(sanitize_degrees_int(30), 30);
        assert_eq!(sanitize_degrees_int(240), 240);
        assert_eq!(sanitize_degrees_int(360), 0);
        assert_eq!(sanitize_degrees_int(-30), 330);
        assert_eq!(sanitize_degrees_int(-750), 330);
        assert_eq!(sanitize_degrees_int(-54_321), 39);

        assert_near(sanitize_degrees_double(30.0), 30.0, 1e-4);
        assert_near(sanitize_degrees_double(240.0), 240.0, 1e-4);
        assert_near(sanitize_degrees_double(360.0), 0.0, 1e-4);
        assert_near(sanitize_degrees_double(-30.0), 330.0, 1e-4);
        assert_near(sanitize_degrees_double(-750.0), 330.0, 1e-4);
        assert_near(sanitize_degrees_double(-54_321.0), 39.0, 1e-4);
        assert_near(sanitize_degrees_double(360.125), 0.125, 1e-4);
        assert_near(sanitize_degrees_double(-11_111.11), 48.89, 1e-4);
    }

    #[test]
    fn matrix_multiply_matches_cpp() {
        let vector_one = matrix_multiply(Vec3::new(1.0, 3.0, 5.0), &MATRIX);
        assert_near(vector_one.a, 22.0, 1e-4);
        assert_near(vector_one.b, -19.0, 1e-4);
        assert_near(vector_one.c, -76.0, 1e-4);

        let vector_two = matrix_multiply(Vec3::new(-11.1, 22.2, -33.3), &MATRIX);
        assert_near(vector_two.a, -66.6, 1e-4);
        assert_near(vector_two.b, 355.2, 1e-4);
        assert_near(vector_two.c, 199.8, 1e-4);
    }

    #[test]
    fn linearized_components_match_cpp() {
        assert_near(linearized(0), 0.0, 1e-4);
        assert_near(linearized(1), 0.030_352_7, 1e-4);
        assert_near(linearized(2), 0.060_705_4, 1e-4);
        assert_near(linearized(8), 0.242_822, 1e-4);
        assert_near(linearized(9), 0.273_174, 1e-4);
        assert_near(linearized(16), 0.518_152, 1e-4);
        assert_near(linearized(32), 1.444_38, 1e-4);
        assert_near(linearized(64), 5.126_95, 1e-4);
        assert_near(linearized(128), 21.586_1, 1e-4);
        assert_near(linearized(255), 100.0, 1e-4);
    }

    #[test]
    fn delinearized_components_match_cpp() {
        assert_eq!(delinearized(0.0), 0);
        assert_eq!(delinearized(0.030_352_7), 1);
        assert_eq!(delinearized(0.060_705_4), 2);
        assert_eq!(delinearized(0.242_822), 8);
        assert_eq!(delinearized(0.273_174), 9);
        assert_eq!(delinearized(0.518_152), 16);
        assert_eq!(delinearized(1.444_38), 32);
        assert_eq!(delinearized(5.126_95), 64);
        assert_eq!(delinearized(21.586_1), 128);
        assert_eq!(delinearized(100.0), 255);
        assert_eq!(delinearized(25.0), 137);
        assert_eq!(delinearized(50.0), 188);
        assert_eq!(delinearized(75.0), 225);
        assert_eq!(delinearized(-1.0), 0);
        assert_eq!(delinearized(-10_000.0), 0);
        assert_eq!(delinearized(101.0), 255);
        assert_eq!(delinearized(10_000.0), 255);
    }

    #[test]
    fn delinearized_is_left_inverse_of_linearized() {
        for component in [0, 1, 2, 8, 9, 16, 32, 64, 128, 255] {
            assert_eq!(delinearized(linearized(component)), i32::from(component));
        }
    }

    #[test]
    fn argb_from_linrgb_matches_cpp() {
        assert_eq!(
            argb_from_linrgb(Vec3::new(25.0, 50.0, 75.0)).to_u32(),
            0xff89_bce1
        );
        assert_eq!(
            argb_from_linrgb(Vec3::new(0.03, 0.06, 0.12)).to_u32(),
            0xff01_0204
        );
    }

    #[test]
    fn lstar_from_argb_matches_cpp() {
        assert_near(lstar_from_argb(Argb::new(0xff89_bce1)), 74.011, 1e-4);
        assert_near(lstar_from_argb(Argb::new(0xff01_0204)), 0.529_651, 1e-4);
    }

    #[test]
    fn hex_from_argb_matches_cpp() {
        assert_eq!(hex_from_argb(Argb::new(0xff89_bce1)), "ff89bce1");
        assert_eq!(hex_from_argb(Argb::new(0xff01_0204)), "ff010204");
    }

    #[test]
    fn int_from_lstar_matches_cpp() {
        for (lstar, argb) in [
            (0.0, 0xff00_0000),
            (0.25, 0xff01_0101),
            (0.5, 0xff02_0202),
            (1.0, 0xff04_0404),
            (2.0, 0xff07_0707),
            (4.0, 0xff0e_0e0e),
            (8.0, 0xff18_1818),
            (25.0, 0xff3b_3b3b),
            (50.0, 0xff77_7777),
            (75.0, 0xffb9_b9b9),
            (99.0, 0xfffc_fcfc),
            (100.0, 0xffff_ffff),
            (-1.0, 0xff00_0000),
            (-2.0, 0xff00_0000),
            (-3.0, 0xff00_0000),
            (-9_999_999.0, 0xff00_0000),
            (101.0, 0xffff_ffff),
            (111.0, 0xffff_ffff),
            (9_999_999.0, 0xffff_ffff),
        ] {
            assert_eq!(int_from_lstar(lstar).to_u32(), argb);
        }
    }

    #[test]
    fn lstar_argb_roundtrip_property() {
        for lstar in [0.0, 1.0, 2.0, 8.0, 25.0, 50.0, 75.0, 99.0, 100.0] {
            assert_near(lstar_from_argb(int_from_lstar(lstar)), lstar, 1.0);
        }
    }

    #[test]
    fn argb_lstar_roundtrip_property_for_grayscale() {
        for argb in [
            0xff00_0000,
            0xff01_0101,
            0xff02_0202,
            0xff11_1111,
            0xff33_3333,
            0xff77_7777,
            0xffbb_bbbb,
            0xfffe_fefe,
            0xffff_ffff,
        ] {
            assert_eq!(
                int_from_lstar(lstar_from_argb(Argb::new(argb))).to_u32(),
                argb
            );
        }
    }

    #[test]
    fn y_from_lstar_matches_cpp() {
        for (lstar, y) in [
            (0.0, 0.0),
            (0.1, 0.011_070_5),
            (0.2, 0.022_141_1),
            (0.3, 0.033_211_6),
            (0.4, 0.044_282_2),
            (0.5, 0.055_352_8),
            (1.0, 0.110_705_6),
            (2.0, 0.221_411_2),
            (3.0, 0.332_116_9),
            (4.0, 0.442_822_5),
            (5.0, 0.553_528_2),
            (8.0, 0.885_645_1),
            (10.0, 1.126_019_9),
            (15.0, 1.908_583_2),
            (20.0, 2.989_052_4),
            (25.0, 4.415_476_7),
            (30.0, 6.235_905_5),
            (40.0, 11.250_973_7),
            (50.0, 18.418_651_8),
            (60.0, 28.123_334_2),
            (70.0, 40.749_415_7),
            (80.0, 56.681_290_7),
            (90.0, 76.303_353_9),
            (95.0, 87.618_329_4),
            (99.0, 97.436_023_9),
            (100.0, 100.0),
        ] {
            assert_near(y_from_lstar(lstar), y, 1e-5);
        }
    }

    #[test]
    fn lstar_from_y_matches_cpp() {
        for (y, lstar) in [
            (0.0, 0.0),
            (0.1, 0.903_296_2),
            (0.2, 1.806_592_5),
            (0.3, 2.709_888_8),
            (0.4, 3.613_185_1),
            (0.5, 4.516_481_4),
            (0.885_645_1, 8.0),
            (1.0, 8.991_442_4),
            (2.0, 15.487_244_3),
            (3.0, 20.043_897_0),
            (4.0, 23.671_441_9),
            (5.0, 26.734_765_3),
            (10.0, 37.842_430_4),
            (15.0, 45.634_197_0),
            (20.0, 51.837_211_5),
            (25.0, 57.075_420_8),
            (30.0, 61.654_222_2),
            (40.0, 69.469_530_7),
            (50.0, 76.069_261_0),
            (60.0, 81.838_189_1),
            (70.0, 86.996_864_2),
            (80.0, 91.684_860_9),
            (90.0, 95.996_768_6),
            (95.0, 98.033_518_4),
            (99.0, 99.612_037_2),
            (100.0, 100.0),
        ] {
            assert_near(lstar_from_y(y), lstar, 1e-5);
        }
    }

    #[test]
    fn y_lstar_roundtrip_property() {
        let mut y = 0.0;
        while y <= 100.0 {
            let lstar = lstar_from_y(y);
            assert_near(y_from_lstar(lstar), y, 1e-8);
            y += 0.1;
        }
    }

    #[test]
    fn lstar_y_roundtrip_property() {
        let mut lstar = 0.0;
        while lstar <= 100.0 {
            let y = y_from_lstar(lstar);
            assert_near(lstar_from_y(y), lstar, 1e-8);
            lstar += 0.1;
        }
    }
}
