#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Argb(u32);

impl Argb {
    pub const BLACK: Self = Self(0xff00_0000);
    pub const WHITE: Self = Self(0xffff_ffff);

    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(0xff00_0000 | ((red as u32) << 16) | ((green as u32) << 8) | blue as u32)
    }

    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn alpha(self) -> u8 {
        ((self.0 & 0xff00_0000) >> 24) as u8
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        ((self.0 & 0x00ff_0000) >> 16) as u8
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        ((self.0 & 0x0000_ff00) >> 8) as u8
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        (self.0 & 0x0000_00ff) as u8
    }

    #[must_use]
    pub const fn is_opaque(self) -> bool {
        self.alpha() == 0xff
    }
}

impl From<u32> for Argb {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Argb> for u32 {
    fn from(value: Argb) -> Self {
        value.to_u32()
    }
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
pub const fn argb_from_rgb(red: i32, green: i32, blue: i32) -> Argb {
    Argb::new(
        0xff00_0000
            | (((red as u32) & 0xff) << 16)
            | (((green as u32) & 0xff) << 8)
            | ((blue as u32) & 0xff),
    )
}

#[must_use]
pub const fn alpha_from_int(argb: Argb) -> i32 {
    argb.alpha() as i32
}

#[must_use]
pub const fn red_from_int(argb: Argb) -> i32 {
    argb.red() as i32
}

#[must_use]
pub const fn green_from_int(argb: Argb) -> i32 {
    argb.green() as i32
}

#[must_use]
pub const fn blue_from_int(argb: Argb) -> i32 {
    argb.blue() as i32
}

#[must_use]
pub const fn is_opaque(argb: Argb) -> bool {
    argb.is_opaque()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argb_from_rgb_returns_correct_value_for_black() {
        assert_eq!(argb_from_rgb(0, 0, 0).to_u32(), 0xff00_0000);
        assert_eq!(argb_from_rgb(0, 0, 0).to_u32(), 4_278_190_080);
    }

    #[test]
    fn argb_from_rgb_returns_correct_value_for_white() {
        assert_eq!(argb_from_rgb(255, 255, 255).to_u32(), 0xffff_ffff);
        assert_eq!(argb_from_rgb(255, 255, 255).to_u32(), 4_294_967_295);
    }

    #[test]
    fn argb_from_rgb_returns_correct_value_for_random_color() {
        assert_eq!(argb_from_rgb(50, 150, 250).to_u32(), 0xff32_96fa);
        assert_eq!(argb_from_rgb(50, 150, 250).to_u32(), 4_281_505_530);
    }

    #[test]
    fn components_match_cpp_helpers() {
        let first = Argb::new(0xff12_3456);
        let second = Argb::new(0xffab_cdef);

        assert_eq!(alpha_from_int(first), 0xff);
        assert_eq!(alpha_from_int(second), 0xff);
        assert_eq!(red_from_int(first), 0x12);
        assert_eq!(red_from_int(second), 0xab);
        assert_eq!(green_from_int(first), 0x34);
        assert_eq!(green_from_int(second), 0xcd);
        assert_eq!(blue_from_int(first), 0x56);
        assert_eq!(blue_from_int(second), 0xef);
    }

    #[test]
    fn opaqueness_matches_alpha_channel() {
        assert!(is_opaque(Argb::new(0xff12_3456)));
        assert!(!is_opaque(Argb::new(0xf012_3456)));
        assert!(!is_opaque(Argb::new(0x0012_3456)));
    }

    #[test]
    fn argb_from_rgb_masks_cpp_style_int_inputs() {
        assert_eq!(argb_from_rgb(-1, 256, 511).to_u32(), 0xffff_00ff);
    }
}
