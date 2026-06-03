use crate::cam::Hct;
use crate::dynamic::DynamicScheme;

#[must_use]
pub fn scheme_expressive(
    source_color_hct: Hct,
    is_dark: bool,
    contrast_level: f64,
) -> DynamicScheme {
    DynamicScheme::expressive(source_color_hct, is_dark, contrast_level)
}
