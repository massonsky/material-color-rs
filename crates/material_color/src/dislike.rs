use crate::cam::Hct;

#[must_use]
pub fn is_disliked(hct: Hct) -> bool {
    let rounded_hue = hct.hue().round();
    let hue_passes = (90.0..=111.0).contains(&rounded_hue);
    let chroma_passes = hct.chroma().round() > 16.0;
    let tone_passes = hct.tone().round() < 65.0;

    hue_passes && chroma_passes && tone_passes
}

#[must_use]
pub fn fix_if_disliked(hct: Hct) -> Hct {
    if is_disliked(hct) {
        Hct::new(hct.hue(), hct.chroma(), 70.0)
    } else {
        hct
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::argb::Argb;

    #[test]
    fn monk_skin_tone_scale_colors_are_liked() {
        for argb in [
            0xfff6_ede4,
            0xfff3_e7db,
            0xfff7_ead0,
            0xffea_daba,
            0xffd7_bd96,
            0xffa0_7e56,
            0xff82_5c43,
            0xff60_4134,
            0xff3a_312a,
            0xff29_2420,
        ] {
            assert!(!is_disliked(Hct::from_argb(Argb::new(argb))));
        }
    }

    #[test]
    fn bile_colors_are_disliked() {
        for argb in [
            0xff95_884b,
            0xff71_6b40,
            0xffb0_8e00,
            0xff4c_4308,
            0xff46_4521,
        ] {
            assert!(is_disliked(Hct::from_argb(Argb::new(argb))));
        }
    }

    #[test]
    fn bile_colors_are_fixed() {
        for argb in [
            0xff95_884b,
            0xff71_6b40,
            0xffb0_8e00,
            0xff4c_4308,
            0xff46_4521,
        ] {
            let bile_color = Hct::from_argb(Argb::new(argb));
            assert!(is_disliked(bile_color));
            assert!(!is_disliked(fix_if_disliked(bile_color)));
        }
    }

    #[test]
    fn tone_67_is_liked() {
        let color = Hct::new(100.0, 50.0, 67.0);

        assert!(!is_disliked(color));
        assert_eq!(fix_if_disliked(color).to_argb(), color.to_argb());
    }
}
