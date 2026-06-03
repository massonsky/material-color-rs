use crate::argb::Argb;
use crate::cam::cam16::Cam;
use crate::cam::hct_solver::solve_to_int;
use crate::utils::lstar_from_argb;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hct {
    hue: f64,
    chroma: f64,
    tone: f64,
    argb: Argb,
}

impl Hct {
    #[must_use]
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self::from_argb(solve_to_int(hue, chroma, tone))
    }

    #[must_use]
    pub fn from_argb(argb: Argb) -> Self {
        let cam = Cam::from_argb(argb);

        Self {
            hue: cam.hue,
            chroma: cam.chroma,
            tone: lstar_from_argb(argb),
            argb,
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
    pub const fn tone(self) -> f64 {
        self.tone
    }

    #[must_use]
    pub const fn to_argb(self) -> Argb {
        self.argb
    }

    pub fn set_hue(&mut self, new_hue: f64) {
        *self = Self::new(new_hue, self.chroma, self.tone);
    }

    pub fn set_chroma(&mut self, new_chroma: f64) {
        *self = Self::new(self.hue, new_chroma, self.tone);
    }

    pub fn set_tone(&mut self, new_tone: f64) {
        *self = Self::new(self.hue, self.chroma, new_tone);
    }
}

impl From<Argb> for Hct {
    fn from(value: Argb) -> Self {
        Self::from_argb(value)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
    use crate::argb::{blue_from_int, green_from_int, red_from_int};

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    fn color_is_on_boundary(argb: Argb) -> bool {
        [
            red_from_int(argb),
            green_from_int(argb),
            blue_from_int(argb),
        ]
        .into_iter()
        .any(|component| component == 0 || component == 255)
    }

    #[test]
    fn limited_to_srgb() {
        let hct = Hct::new(120.0, 200.0, 50.0);
        let argb = hct.to_argb();

        assert_eq!(Cam::from_argb(argb).hue, hct.hue());
        assert_eq!(Cam::from_argb(argb).chroma, hct.chroma());
        assert_eq!(lstar_from_argb(argb), hct.tone());
    }

    #[test]
    fn truncates_colors() {
        let mut hct = Hct::new(120.0, 60.0, 50.0);
        let chroma = hct.chroma();
        assert!(chroma < 60.0);

        hct.set_tone(180.0);
        assert!(hct.chroma() < chroma);
    }

    #[test]
    fn hct_from_argb_uses_cam_hue_chroma_and_lstar_tone() {
        let hct = Hct::from_argb(Argb::new(0xff00_00ff));

        assert_near(hct.hue(), 282.788, 0.001);
        assert_near(hct.chroma(), 87.230, 0.001);
        assert_near(hct.tone(), 32.302_586, 0.001);
        assert_eq!(hct.to_argb(), Argb::new(0xff00_00ff));
    }

    #[test]
    fn mutators_resolve_new_srgb_color() {
        let mut hct = Hct::from_argb(Argb::new(0xffff_0000));
        hct.set_tone(80.0);

        assert_near(hct.tone(), 80.0, 0.5);
        assert_near(hct.hue(), 27.408, 4.0);

        hct.set_hue(140.0);
        assert_near(hct.hue(), 140.0, 4.0);

        hct.set_chroma(40.0);
        assert!(hct.chroma() <= 42.5);
    }

    #[test]
    fn correctness_grid_matches_cpp_contract() {
        for hue in [15, 45, 75, 105, 135, 165, 195, 225, 255, 285, 315, 345] {
            for chroma in [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
                for tone in [20, 30, 40, 50, 60, 70, 80] {
                    let color = Hct::new(f64::from(hue), f64::from(chroma), f64::from(tone));

                    if chroma > 0 {
                        assert_near(color.hue(), f64::from(hue), 4.0);
                    }

                    assert!(
                        color.chroma() < f64::from(chroma) + 2.5,
                        "chroma {} >= requested {} + 2.5",
                        color.chroma(),
                        chroma
                    );

                    if color.chroma() < f64::from(chroma) - 2.5 {
                        assert!(
                            color_is_on_boundary(color.to_argb()),
                            "expected boundary color for {color:?}"
                        );
                    }

                    assert_near(color.tone(), f64::from(tone), 0.5);
                }
            }
        }
    }
}
