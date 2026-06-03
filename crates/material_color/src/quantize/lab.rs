use crate::argb::{Argb, argb_from_rgb};
use crate::utils::{WHITE_POINT_D65, delinearized, linearized};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Lab {
    #[must_use]
    pub const fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }

    #[must_use]
    pub fn delta_e(self, other: Self) -> f64 {
        let delta_l = self.l - other.l;
        let delta_a = self.a - other.a;
        let delta_b = self.b - other.b;

        delta_l * delta_l + delta_a * delta_a + delta_b * delta_b
    }
}

#[must_use]
#[allow(clippy::similar_names)]
pub fn int_from_lab(lab: Lab) -> Argb {
    const E: f64 = 216.0 / 24_389.0;
    const KAPPA: f64 = 24_389.0 / 27.0;
    const KE: f64 = 8.0;

    let fy = (lab.l + 16.0) / 116.0;
    let fx = (lab.a / 500.0) + fy;
    let fz = fy - (lab.b / 200.0);
    let fx3 = fx * fx * fx;
    let x_normalized = if fx3 > E {
        fx3
    } else {
        (116.0 * fx - 16.0) / KAPPA
    };
    let y_normalized = if lab.l > KE {
        fy * fy * fy
    } else {
        lab.l / KAPPA
    };
    let fz3 = fz * fz * fz;
    let z_normalized = if fz3 > E {
        fz3
    } else {
        (116.0 * fz - 16.0) / KAPPA
    };
    let x = x_normalized * WHITE_POINT_D65[0];
    let y = y_normalized * WHITE_POINT_D65[1];
    let z = z_normalized * WHITE_POINT_D65[2];

    let red_l = 3.2406 * x - 1.5372 * y - 0.4986 * z;
    let green_l = -0.9689 * x + 1.8758 * y + 0.0415 * z;
    let blue_l = 0.0557 * x - 0.2040 * y + 1.0570 * z;

    argb_from_rgb(
        delinearized(red_l),
        delinearized(green_l),
        delinearized(blue_l),
    )
}

#[must_use]
#[allow(clippy::similar_names)]
pub fn lab_from_argb(argb: Argb) -> Lab {
    let red_l = linearized(argb.red());
    let green_l = linearized(argb.green());
    let blue_l = linearized(argb.blue());
    let x = 0.412_338_95 * red_l + 0.357_620_64 * green_l + 0.180_510_42 * blue_l;
    let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
    let z = 0.019_321_41 * red_l + 0.119_163_82 * green_l + 0.950_344_78 * blue_l;
    let fy = lab_f(y / WHITE_POINT_D65[1]);
    let fx = lab_f(x / WHITE_POINT_D65[0]);
    let fz = lab_f(z / WHITE_POINT_D65[2]);

    Lab {
        l: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

fn lab_f(normalized: f64) -> f64 {
    const E: f64 = 216.0 / 24_389.0;
    const KAPPA: f64 = 24_389.0 / 27.0;

    if normalized > E {
        normalized.powf(1.0 / 3.0)
    } else {
        (KAPPA * normalized + 16.0) / 116.0
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
    fn lab_from_argb_matches_representative_values() {
        let red = lab_from_argb(Argb::new(0xffff_0000));
        assert_near(red.l, 53.233, 0.001);
        assert_near(red.a, 80.091, 0.001);
        assert_near(red.b, 67.201, 0.001);

        let blue = lab_from_argb(Argb::new(0xff00_00ff));
        assert_near(blue.l, 32.302, 0.001);
        assert_near(blue.a, 79.202, 0.001);
        assert_near(blue.b, -107.853, 0.001);
    }

    #[test]
    fn lab_round_trips_primary_colors() {
        for argb in [
            Argb::new(0xffff_0000),
            Argb::new(0xff00_ff00),
            Argb::new(0xff00_00ff),
            Argb::new(0xff14_1216),
        ] {
            assert_eq!(int_from_lab(lab_from_argb(argb)), argb);
        }
    }
}
