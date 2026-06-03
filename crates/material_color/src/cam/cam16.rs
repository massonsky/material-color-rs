#![allow(clippy::many_single_char_names)]

use crate::argb::{Argb, argb_from_rgb};
use crate::cam::hct_solver::solve_to_int;
use crate::cam::viewing_conditions::ViewingConditions;
use crate::utils::{PI, delinearized, linearized, sanitize_degrees_double, signum};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cam {
    pub hue: f64,
    pub chroma: f64,
    pub j: f64,
    pub q: f64,
    pub m: f64,
    pub s: f64,
    pub jstar: f64,
    pub astar: f64,
    pub bstar: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Cam {
    #[must_use]
    pub fn from_argb(argb: Argb) -> Self {
        Self::from_argb_and_viewing_conditions(argb, ViewingConditions::DEFAULT)
    }

    #[must_use]
    pub fn from_argb_and_viewing_conditions(
        argb: Argb,
        viewing_conditions: ViewingConditions,
    ) -> Self {
        Self::from_xyz_and_viewing_conditions(xyz_from_argb(argb), viewing_conditions)
    }

    #[must_use]
    pub fn from_xyz_and_viewing_conditions(
        xyz: Xyz,
        viewing_conditions: ViewingConditions,
    ) -> Self {
        let r_c = 0.401_288 * xyz.x + 0.650_173 * xyz.y - 0.051_461 * xyz.z;
        let g_c = -0.250_268 * xyz.x + 1.204_414 * xyz.y + 0.045_854 * xyz.z;
        let b_c = -0.002_079 * xyz.x + 0.048_952 * xyz.y + 0.953_127 * xyz.z;

        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        let r_af = (viewing_conditions.fl * r_d.abs() / 100.0).powf(0.42);
        let g_af = (viewing_conditions.fl * g_d.abs() / 100.0).powf(0.42);
        let b_af = (viewing_conditions.fl * b_d.abs() / 100.0).powf(0.42);
        let r_a = f64::from(signum(r_d)) * 400.0 * r_af / (r_af + 27.13);
        let g_a = f64::from(signum(g_d)) * 400.0 * g_af / (g_af + 27.13);
        let b_a = f64::from(signum(b_d)) * 400.0 * b_af / (b_af + 27.13);

        let a = (11.0 * r_a - 12.0 * g_a + b_a) / 11.0;
        let b = (r_a + g_a - 2.0 * b_a) / 9.0;
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;

        let hue = sanitize_degrees_double(b.atan2(a) * 180.0 / PI);
        let hue_radians = hue * PI / 180.0;
        let ac = p2 * viewing_conditions.nbb;
        let j =
            100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * viewing_conditions.fl_root;
        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * ((hue_prime * PI / 180.0 + 2.0).cos() + 3.8);
        let p1 = 50_000.0 / 13.0 * e_hue * viewing_conditions.n_c * viewing_conditions.ncb;
        let t = p1 * (a * a + b * b).sqrt() / (u + 0.305);
        let alpha = t.powf(0.9)
            * (1.64 - 0.29_f64.powf(viewing_conditions.background_y_to_white_point_y)).powf(0.73);
        let chroma = alpha * (j / 100.0).sqrt();
        let m = chroma * viewing_conditions.fl_root;
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();
        let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        let mstar = (1.0 + 0.0228 * m).ln() / 0.0228;
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Self {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    #[must_use]
    pub fn from_jch_and_viewing_conditions(
        j: f64,
        chroma: f64,
        hue: f64,
        viewing_conditions: ViewingConditions,
    ) -> Self {
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * viewing_conditions.fl_root;
        let m = chroma * viewing_conditions.fl_root;
        let alpha = chroma / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();
        let hue_radians = hue * PI / 180.0;
        let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        let mstar = (1.0 + 0.0228 * m).ln() / 0.0228;
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Self {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    #[must_use]
    pub fn from_ucs_and_viewing_conditions(
        jstar: f64,
        astar: f64,
        bstar: f64,
        viewing_conditions: ViewingConditions,
    ) -> Self {
        let m = (astar * astar + bstar * bstar).sqrt();
        let m_2 = ((m * 0.0228).exp() - 1.0) / 0.0228;
        let chroma = m_2 / viewing_conditions.fl_root;
        let mut hue = bstar.atan2(astar) * 180.0 / PI;
        if hue < 0.0 {
            hue += 360.0;
        }
        let j = jstar / (1.0 - (jstar - 100.0) * 0.007);

        Self::from_jch_and_viewing_conditions(j, chroma, hue, viewing_conditions)
    }

    #[must_use]
    pub fn to_argb(self) -> Argb {
        self.to_argb_with_viewing_conditions(ViewingConditions::DEFAULT)
    }

    #[must_use]
    pub fn to_argb_with_viewing_conditions(self, viewing_conditions: ViewingConditions) -> Argb {
        let alpha = if self.chroma == 0.0 || self.j == 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };
        let t = (alpha
            / (1.64 - 0.29_f64.powf(viewing_conditions.background_y_to_white_point_y)).powf(0.73))
        .powf(1.0 / 0.9);
        let h_rad = self.hue * PI / 180.0;
        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = viewing_conditions.aw
            * (self.j / 100.0).powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p1 = e_hue * (50_000.0 / 13.0) * viewing_conditions.n_c * viewing_conditions.ncb;
        let p2 = ac / viewing_conditions.nbb;
        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_c = inverse_chromatic_adaptation(r_a, viewing_conditions.fl);
        let g_c = inverse_chromatic_adaptation(g_a, viewing_conditions.fl);
        let b_c = inverse_chromatic_adaptation(b_a, viewing_conditions.fl);
        let r_x = r_c / viewing_conditions.rgb_d[0];
        let g_x = g_c / viewing_conditions.rgb_d[1];
        let b_x = b_c / viewing_conditions.rgb_d[2];
        let x = 1.862_067_86 * r_x - 1.011_254_63 * g_x + 0.149_186_77 * b_x;
        let y = 0.387_526_54 * r_x + 0.621_447_44 * g_x - 0.008_973_98 * b_x;
        let z = -0.015_841_50 * r_x - 0.034_122_94 * g_x + 1.049_964_44 * b_x;

        argb_from_xyz(Xyz::new(x, y, z))
    }

    #[must_use]
    pub fn distance(self, other: Self) -> f64 {
        let d_j = self.jstar - other.jstar;
        let d_a = self.astar - other.astar;
        let d_b = self.bstar - other.bstar;
        let d_e_prime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();

        1.41 * d_e_prime.powf(0.63)
    }
}

#[must_use]
pub fn cam_from_int(argb: Argb) -> Cam {
    Cam::from_argb(argb)
}

#[must_use]
pub fn cam_from_int_and_viewing_conditions(
    argb: Argb,
    viewing_conditions: ViewingConditions,
) -> Cam {
    Cam::from_argb_and_viewing_conditions(argb, viewing_conditions)
}

#[must_use]
pub fn cam_from_ucs_and_viewing_conditions(
    jstar: f64,
    astar: f64,
    bstar: f64,
    viewing_conditions: ViewingConditions,
) -> Cam {
    Cam::from_ucs_and_viewing_conditions(jstar, astar, bstar, viewing_conditions)
}

#[must_use]
pub fn cam_from_xyz_and_viewing_conditions(
    x: f64,
    y: f64,
    z: f64,
    viewing_conditions: ViewingConditions,
) -> Cam {
    Cam::from_xyz_and_viewing_conditions(Xyz::new(x, y, z), viewing_conditions)
}

#[must_use]
pub fn int_from_cam(cam: Cam) -> Argb {
    cam.to_argb()
}

#[must_use]
pub fn int_from_hcl(hue: f64, chroma: f64, lstar: f64) -> Argb {
    solve_to_int(hue, chroma, lstar)
}

#[must_use]
pub fn cam_distance(a: Cam, b: Cam) -> f64 {
    a.distance(b)
}

fn inverse_chromatic_adaptation(component: f64, fl: f64) -> f64 {
    let base = (27.13 * component.abs()) / (400.0 - component.abs());

    f64::from(signum(component)) * 100.0 / fl * base.max(0.0).powf(1.0 / 0.42)
}

fn xyz_from_argb(argb: Argb) -> Xyz {
    let red_l = linearized(argb.red());
    let green_l = linearized(argb.green());
    let blue_l = linearized(argb.blue());

    Xyz {
        x: 0.412_338_95 * red_l + 0.357_620_64 * green_l + 0.180_510_42 * blue_l,
        y: 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l,
        z: 0.019_321_41 * red_l + 0.119_163_82 * green_l + 0.950_344_78 * blue_l,
    }
}

fn argb_from_xyz(xyz: Xyz) -> Argb {
    let r_l = 3.2406 * xyz.x - 1.5372 * xyz.y - 0.4986 * xyz.z;
    let g_l = -0.9689 * xyz.x + 1.8758 * xyz.y + 0.0415 * xyz.z;
    let b_l = 0.0557 * xyz.x - 0.2040 * xyz.y + 1.0570 * xyz.z;

    argb_from_rgb(delinearized(r_l), delinearized(g_l), delinearized(b_l))
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
    fn red_matches_cpp_cam16_values() {
        let cam = Cam::from_argb(Argb::new(0xffff_0000));

        assert_near(cam.hue, 27.408, 0.001);
        assert_near(cam.chroma, 113.357, 0.001);
        assert_near(cam.j, 46.445, 0.001);
        assert_near(cam.m, 89.494, 0.001);
        assert_near(cam.s, 91.889, 0.001);
        assert_near(cam.q, 105.988, 0.001);
    }

    #[test]
    fn green_matches_cpp_cam16_values() {
        let cam = Cam::from_argb(Argb::new(0xff00_ff00));

        assert_near(cam.hue, 142.139, 0.001);
        assert_near(cam.chroma, 108.410, 0.001);
        assert_near(cam.j, 79.331, 0.001);
        assert_near(cam.m, 85.587, 0.001);
        assert_near(cam.s, 78.604, 0.001);
        assert_near(cam.q, 138.520, 0.001);
    }

    #[test]
    fn blue_matches_cpp_cam16_values() {
        let cam = Cam::from_argb(Argb::new(0xff00_00ff));

        assert_near(cam.hue, 282.788, 0.001);
        assert_near(cam.chroma, 87.230, 0.001);
        assert_near(cam.j, 25.465, 0.001);
        assert_near(cam.m, 68.867, 0.001);
        assert_near(cam.s, 93.674, 0.001);
        assert_near(cam.q, 78.481, 0.001);
    }

    #[test]
    fn white_matches_cpp_cam16_values() {
        let cam = Cam::from_argb(Argb::new(0xffff_ffff));

        assert_near(cam.hue, 209.492, 0.001);
        assert_near(cam.chroma, 2.869, 0.001);
        assert_near(cam.j, 100.0, 0.001);
        assert_near(cam.m, 2.265, 0.001);
        assert_near(cam.s, 12.068, 0.001);
        assert_near(cam.q, 155.521, 0.001);
    }

    #[test]
    fn black_matches_cpp_cam16_values() {
        let cam = Cam::from_argb(Argb::new(0xff00_0000));

        assert_near(cam.hue, 0.0, 0.001);
        assert_near(cam.chroma, 0.0, 0.001);
        assert_near(cam.j, 0.0, 0.001);
        assert_near(cam.m, 0.0, 0.001);
        assert_near(cam.s, 0.0, 0.001);
        assert_near(cam.q, 0.0, 0.001);
    }

    #[test]
    fn primaries_round_trip_through_cam16() {
        for color in [0xffff_0000, 0xff00_ff00, 0xff00_00ff] {
            let argb = Argb::new(color);
            assert_eq!(Cam::from_argb(argb).to_argb(), argb);
        }
    }
}
