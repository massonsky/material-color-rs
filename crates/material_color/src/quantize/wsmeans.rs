#![allow(clippy::cast_precision_loss)]

use std::collections::{BTreeMap, HashMap};

use crate::argb::Argb;
use crate::quantize::lab::{Lab, int_from_lab, lab_from_argb};

const MAX_ITERATIONS: usize = 100;
const MIN_DELTA_E: f64 = 3.0;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuantizerResult {
    pub color_to_count: BTreeMap<Argb, u32>,
    pub input_pixel_to_cluster_pixel: BTreeMap<Argb, Argb>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Swatch {
    argb: Argb,
    population: u32,
}

#[derive(Clone, Copy, Debug, Default)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    const RAND_MAX: u64 = 0x7fff_ffff;

    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u31(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        (self.state / 65_536) % (Self::RAND_MAX + 1)
    }

    fn next_f64(&mut self) -> f64 {
        self.next_u31() as f64 / Self::RAND_MAX as f64
    }
}

#[must_use]
#[allow(clippy::similar_names, clippy::too_many_lines)]
pub fn quantize_wsmeans(
    input_pixels: &[Argb],
    starting_clusters: &[Argb],
    max_colors: u16,
) -> QuantizerResult {
    if max_colors == 0 || input_pixels.is_empty() {
        return QuantizerResult::default();
    }

    let max_colors = max_colors.min(256);
    let mut pixel_to_count = HashMap::<Argb, u32>::new();
    let mut pixels = Vec::with_capacity(input_pixels.len());
    let mut points = Vec::with_capacity(input_pixels.len());

    for &pixel in input_pixels {
        if let Some(count) = pixel_to_count.get_mut(&pixel) {
            *count += 1;
        } else {
            pixels.push(pixel);
            points.push(lab_from_argb(pixel));
            pixel_to_count.insert(pixel, 1);
        }
    }

    let mut cluster_count = usize::from(max_colors).min(points.len());
    if !starting_clusters.is_empty() {
        cluster_count = cluster_count.min(starting_clusters.len());
    }
    if cluster_count == 0 {
        return QuantizerResult::default();
    }

    let mut clusters = starting_clusters
        .iter()
        .take(cluster_count)
        .map(|&argb| lab_from_argb(argb))
        .collect::<Vec<_>>();

    let additional_clusters_needed = cluster_count.saturating_sub(clusters.len());
    if starting_clusters.is_empty() && additional_clusters_needed > 0 {
        let mut rng = DeterministicRng::new(42_688);
        for _ in 0..additional_clusters_needed {
            clusters.push(Lab::new(
                rng.next_f64() * 100.0,
                rng.next_f64() * 200.0 - 100.0,
                rng.next_f64() * 200.0 - 100.0,
            ));
        }
    }

    let mut rng = DeterministicRng::new(42_688);
    let mut cluster_indices = (0..points.len())
        .map(|_| {
            #[allow(clippy::cast_possible_truncation)]
            let index = rng.next_u31() as usize % cluster_count;
            index
        })
        .collect::<Vec<_>>();

    let mut distance_to_index_matrix = vec![vec![0.0; cluster_count]; cluster_count];
    let mut pixel_count_sums = vec![0_u32; cluster_count];

    for iteration in 0..MAX_ITERATIONS {
        for first in 0..cluster_count {
            distance_to_index_matrix[first][first] = 0.0;
            for second in (first + 1)..cluster_count {
                let distance = clusters[first].delta_e(clusters[second]);
                distance_to_index_matrix[second][first] = distance;
                distance_to_index_matrix[first][second] = distance;
            }
        }

        let mut color_moved = false;
        for (index, point) in points.iter().enumerate() {
            let previous_cluster_index = cluster_indices[index];
            let previous_cluster = clusters[previous_cluster_index];
            let previous_distance = point.delta_e(previous_cluster);
            let mut minimum_distance = previous_distance;
            let mut new_cluster_index = None;

            for (candidate_index, candidate_cluster) in clusters.iter().enumerate() {
                if distance_to_index_matrix[previous_cluster_index][candidate_index]
                    >= 4.0 * previous_distance
                {
                    continue;
                }
                let distance = point.delta_e(*candidate_cluster);
                if distance < minimum_distance {
                    minimum_distance = distance;
                    new_cluster_index = Some(candidate_index);
                }
            }

            if let Some(new_cluster_index) = new_cluster_index {
                let distance_change = (minimum_distance.sqrt() - previous_distance.sqrt()).abs();
                if distance_change > MIN_DELTA_E {
                    color_moved = true;
                    cluster_indices[index] = new_cluster_index;
                }
            }
        }

        if !color_moved && iteration != 0 {
            break;
        }

        let mut component_l_sums = vec![0.0; cluster_count];
        let mut component_a_sums = vec![0.0; cluster_count];
        let mut component_b_sums = vec![0.0; cluster_count];
        pixel_count_sums.fill(0);

        for (index, point) in points.iter().enumerate() {
            let cluster_index = cluster_indices[index];
            let count = pixel_to_count[&pixels[index]];
            let count_f64 = f64::from(count);

            pixel_count_sums[cluster_index] += count;
            component_l_sums[cluster_index] += point.l * count_f64;
            component_a_sums[cluster_index] += point.a * count_f64;
            component_b_sums[cluster_index] += point.b * count_f64;
        }

        for index in 0..cluster_count {
            let count = pixel_count_sums[index];
            if count == 0 {
                clusters[index] = Lab::default();
                continue;
            }
            let count = f64::from(count);
            clusters[index] = Lab::new(
                component_l_sums[index] / count,
                component_a_sums[index] / count,
                component_b_sums[index] / count,
            );
        }
    }

    let mut swatches = Vec::<Swatch>::new();
    let mut all_cluster_argbs = Vec::with_capacity(cluster_count);
    for (index, cluster) in clusters.iter().enumerate() {
        let possible_new_cluster = int_from_lab(*cluster);
        all_cluster_argbs.push(possible_new_cluster);

        let count = pixel_count_sums[index];
        if count == 0 {
            continue;
        }
        if let Some(swatch) = swatches
            .iter_mut()
            .find(|swatch| swatch.argb == possible_new_cluster)
        {
            swatch.population += count;
        } else {
            swatches.push(Swatch {
                argb: possible_new_cluster,
                population: count,
            });
        }
    }
    swatches.sort_by(|left, right| {
        right
            .population
            .cmp(&left.population)
            .then_with(|| left.argb.cmp(&right.argb))
    });

    let color_to_count = swatches
        .into_iter()
        .map(|swatch| (swatch.argb, swatch.population))
        .collect::<BTreeMap<_, _>>();

    let input_pixel_to_cluster_pixel = points
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let pixel = pixels[index];
            let cluster_index = cluster_indices[index];
            let cluster_argb = all_cluster_argbs[cluster_index];
            (pixel, cluster_argb)
        })
        .collect::<BTreeMap<_, _>>();

    QuantizerResult {
        color_to_count,
        input_pixel_to_cluster_pixel,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_primary_color_matches_cpp() {
        let red = quantize_wsmeans(&[Argb::new(0xffff_0000)], &[], 256);
        assert_eq!(red.color_to_count.len(), 1);
        assert_eq!(red.color_to_count[&Argb::new(0xffff_0000)], 1);

        let green = quantize_wsmeans(&[Argb::new(0xff00_ff00)], &[], 256);
        assert_eq!(green.color_to_count.len(), 1);
        assert_eq!(green.color_to_count[&Argb::new(0xff00_ff00)], 1);

        let blue = quantize_wsmeans(&[Argb::new(0xff00_00ff)], &[], 256);
        assert_eq!(blue.color_to_count.len(), 1);
        assert_eq!(blue.color_to_count[&Argb::new(0xff00_00ff)], 1);
    }

    #[test]
    fn repeated_color_keeps_population() {
        let pixels = vec![Argb::new(0xff00_00ff); 5];
        let result = quantize_wsmeans(&pixels, &[], 256);

        assert_eq!(result.color_to_count.len(), 1);
        assert_eq!(result.color_to_count[&Argb::new(0xff00_00ff)], 5);
    }
}
