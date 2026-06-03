use crate::argb::Argb;
use crate::cam::{Cam, Hct, ViewingConditions};
use crate::utils::{diff_degrees, rotation_direction, sanitize_degrees_double};

#[must_use]
pub fn harmonize(design_color: Argb, key_color: Argb) -> Argb {
    let mut from_hct = Hct::from_argb(design_color);
    let to_hct = Hct::from_argb(key_color);
    let difference_degrees = diff_degrees(from_hct.hue(), to_hct.hue());
    let rotation_degrees = (difference_degrees * 0.5).min(15.0);
    let output_hue = sanitize_degrees_double(
        from_hct.hue() + rotation_degrees * rotation_direction(from_hct.hue(), to_hct.hue()),
    );

    from_hct.set_hue(output_hue);
    from_hct.to_argb()
}

#[must_use]
pub fn hct_hue(from: Argb, to: Argb, amount: f64) -> Argb {
    let ucs = cam16_ucs(from, to, amount);
    let ucs_hct = Hct::from_argb(ucs);
    let mut from_hct = Hct::from_argb(from);
    from_hct.set_hue(ucs_hct.hue());

    from_hct.to_argb()
}

#[must_use]
pub fn cam16_ucs(from: Argb, to: Argb, amount: f64) -> Argb {
    let from_cam = Cam::from_argb(from);
    let to_cam = Cam::from_argb(to);

    let jstar = from_cam.jstar + (to_cam.jstar - from_cam.jstar) * amount;
    let astar = from_cam.astar + (to_cam.astar - from_cam.astar) * amount;
    let bstar = from_cam.bstar + (to_cam.bstar - from_cam.bstar) * amount;
    let blended =
        Cam::from_ucs_and_viewing_conditions(jstar, astar, bstar, ViewingConditions::DEFAULT);

    blended.to_argb()
}

pub use cam16_ucs as blend_cam16_ucs;
pub use harmonize as blend_harmonize;
pub use hct_hue as blend_hct_hue;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hex_from_argb;

    #[test]
    fn hct_hue_red_to_blue_matches_cpp() {
        let blended = hct_hue(Argb::new(0xffff_0000), Argb::new(0xff00_00ff), 0.8);

        assert_eq!(hex_from_argb(blended), "ff905eff");
    }
}
