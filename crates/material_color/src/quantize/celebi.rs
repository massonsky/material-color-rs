use crate::argb::Argb;
use crate::quantize::wsmeans::{QuantizerResult, quantize_wsmeans};
use crate::quantize::wu::quantize_wu;

#[must_use]
pub fn quantize_celebi(pixels: &[Argb], max_colors: u16) -> QuantizerResult {
    if max_colors == 0 || pixels.is_empty() {
        return QuantizerResult::default();
    }

    let max_colors = max_colors.min(256);
    let opaque_pixels = pixels
        .iter()
        .copied()
        .filter(|pixel| pixel.is_opaque())
        .collect::<Vec<_>>();

    if opaque_pixels.is_empty() {
        return QuantizerResult::default();
    }

    let starting_clusters = quantize_wu(&opaque_pixels, max_colors);
    quantize_wsmeans(&opaque_pixels, &starting_clusters, max_colors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_primary_color_matches_cpp() {
        let red = quantize_celebi(&[Argb::new(0xffff_0000)], 256);
        assert_eq!(red.color_to_count.len(), 1);
        assert_eq!(red.color_to_count[&Argb::new(0xffff_0000)], 1);

        let green = quantize_celebi(&[Argb::new(0xff00_ff00)], 256);
        assert_eq!(green.color_to_count.len(), 1);
        assert_eq!(green.color_to_count[&Argb::new(0xff00_ff00)], 1);

        let blue = quantize_celebi(&[Argb::new(0xff00_00ff)], 256);
        assert_eq!(blue.color_to_count.len(), 1);
        assert_eq!(blue.color_to_count[&Argb::new(0xff00_00ff)], 1);
    }

    #[test]
    fn repeated_colors_keep_populations() {
        let pixels = [
            Argb::new(0xffff_0000),
            Argb::new(0xffff_0000),
            Argb::new(0xff00_ff00),
            Argb::new(0xff00_ff00),
            Argb::new(0xff00_ff00),
        ];
        let result = quantize_celebi(&pixels, 256);

        assert_eq!(result.color_to_count.len(), 2);
        assert_eq!(result.color_to_count[&Argb::new(0xffff_0000)], 2);
        assert_eq!(result.color_to_count[&Argb::new(0xff00_ff00)], 3);
    }

    #[test]
    fn transparent_pixels_are_ignored() {
        let result = quantize_celebi(&[Argb::new(0x20f9_3013)], 1);

        assert!(result.color_to_count.is_empty());
        assert!(result.input_pixel_to_cluster_pixel.is_empty());
    }
}
