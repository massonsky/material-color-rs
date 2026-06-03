#![allow(clippy::cast_precision_loss)]

use crate::argb::{Argb, argb_from_rgb};

const INDEX_BITS: usize = 5;
const INDEX_COUNT: usize = (1 << INDEX_BITS) + 1;
const TOTAL_SIZE: usize = INDEX_COUNT * INDEX_COUNT * INDEX_COUNT;
const MAX_COLORS: usize = 256;

type IntArray = Vec<i64>;
type DoubleArray = Vec<f64>;

#[derive(Clone, Copy, Debug, Default)]
struct Box3d {
    r0: usize,
    r1: usize,
    g0: usize,
    g1: usize,
    b0: usize,
    b1: usize,
    volume: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Red,
    Green,
    Blue,
}

#[must_use]
pub fn quantize_wu(pixels: &[Argb], max_colors: u16) -> Vec<Argb> {
    if max_colors == 0 || max_colors > 256 || pixels.is_empty() {
        return Vec::new();
    }

    let mut weights = vec![0; TOTAL_SIZE];
    let mut moments_red = vec![0; TOTAL_SIZE];
    let mut moments_green = vec![0; TOTAL_SIZE];
    let mut moments_blue = vec![0; TOTAL_SIZE];
    let mut moments = vec![0.0; TOTAL_SIZE];
    construct_histogram(
        pixels,
        &mut weights,
        &mut moments_red,
        &mut moments_green,
        &mut moments_blue,
        &mut moments,
    );
    compute_moments(
        &mut weights,
        &mut moments_red,
        &mut moments_green,
        &mut moments_blue,
        &mut moments,
    );

    let mut cubes = vec![Box3d::default(); MAX_COLORS];
    cubes[0].r1 = INDEX_COUNT - 1;
    cubes[0].g1 = INDEX_COUNT - 1;
    cubes[0].b1 = INDEX_COUNT - 1;

    let mut volume_variance = vec![0.0; MAX_COLORS];
    let mut generated_color_count = usize::from(max_colors);
    let mut next = 0;
    let mut index = 1;
    while index < usize::from(max_colors) {
        let (left, right) = cubes.split_at_mut(index);
        if cut(
            &mut left[next],
            &mut right[0],
            &weights,
            &moments_red,
            &moments_green,
            &moments_blue,
        ) {
            volume_variance[next] = if cubes[next].volume > 1 {
                variance(
                    cubes[next],
                    &weights,
                    &moments_red,
                    &moments_green,
                    &moments_blue,
                    &moments,
                )
            } else {
                0.0
            };
            volume_variance[index] = if cubes[index].volume > 1 {
                variance(
                    cubes[index],
                    &weights,
                    &moments_red,
                    &moments_green,
                    &moments_blue,
                    &moments,
                )
            } else {
                0.0
            };
        } else {
            volume_variance[next] = 0.0;
            index -= 1;
        }

        next = 0;
        let mut temp = volume_variance[0];
        for (candidate, candidate_variance) in
            volume_variance.iter().enumerate().take(index + 1).skip(1)
        {
            if *candidate_variance > temp {
                temp = *candidate_variance;
                next = candidate;
            }
        }

        if temp <= 0.0 {
            generated_color_count = index + 1;
            break;
        }
        index += 1;
    }

    let mut out_colors = Vec::new();
    for cube in cubes.into_iter().take(generated_color_count) {
        let weight = volume(cube, &weights);
        if weight > 0 {
            #[allow(clippy::cast_possible_truncation)]
            let red = (volume(cube, &moments_red) / weight) as i32;
            #[allow(clippy::cast_possible_truncation)]
            let green = (volume(cube, &moments_green) / weight) as i32;
            #[allow(clippy::cast_possible_truncation)]
            let blue = (volume(cube, &moments_blue) / weight) as i32;
            out_colors.push(argb_from_rgb(red, green, blue));
        }
    }

    out_colors
}

fn construct_histogram(
    pixels: &[Argb],
    weights: &mut IntArray,
    moments_red: &mut IntArray,
    moments_green: &mut IntArray,
    moments_blue: &mut IntArray,
    moments: &mut DoubleArray,
) {
    let bits_to_remove = 8 - INDEX_BITS;
    for &pixel in pixels {
        let red = i64::from(pixel.red());
        let green = i64::from(pixel.green());
        let blue = i64::from(pixel.blue());
        let index_r = (usize::from(pixel.red()) >> bits_to_remove) + 1;
        let index_g = (usize::from(pixel.green()) >> bits_to_remove) + 1;
        let index_b = (usize::from(pixel.blue()) >> bits_to_remove) + 1;
        let index = get_index(index_r, index_g, index_b);

        weights[index] += 1;
        moments_red[index] += red;
        moments_green[index] += green;
        moments_blue[index] += blue;
        moments[index] += (red * red + green * green + blue * blue) as f64;
    }
}

fn compute_moments(
    weights: &mut IntArray,
    moments_red: &mut IntArray,
    moments_green: &mut IntArray,
    moments_blue: &mut IntArray,
    moments: &mut DoubleArray,
) {
    for red in 1..INDEX_COUNT {
        let mut area = [0; INDEX_COUNT];
        let mut area_red = [0; INDEX_COUNT];
        let mut area_green = [0; INDEX_COUNT];
        let mut area_blue = [0; INDEX_COUNT];
        let mut area_2 = [0.0; INDEX_COUNT];

        for green in 1..INDEX_COUNT {
            let mut line = 0;
            let mut line_red = 0;
            let mut line_green = 0;
            let mut line_blue = 0;
            let mut line_2 = 0.0;

            for blue in 1..INDEX_COUNT {
                let index = get_index(red, green, blue);
                line += weights[index];
                line_red += moments_red[index];
                line_green += moments_green[index];
                line_blue += moments_blue[index];
                line_2 += moments[index];

                area[blue] += line;
                area_red[blue] += line_red;
                area_green[blue] += line_green;
                area_blue[blue] += line_blue;
                area_2[blue] += line_2;

                let previous_index = get_index(red - 1, green, blue);
                weights[index] = weights[previous_index] + area[blue];
                moments_red[index] = moments_red[previous_index] + area_red[blue];
                moments_green[index] = moments_green[previous_index] + area_green[blue];
                moments_blue[index] = moments_blue[previous_index] + area_blue[blue];
                moments[index] = moments[previous_index] + area_2[blue];
            }
        }
    }
}

fn cut(
    box1: &mut Box3d,
    box2: &mut Box3d,
    weights: &IntArray,
    moments_red: &IntArray,
    moments_green: &IntArray,
    moments_blue: &IntArray,
) -> bool {
    let whole_red = volume(*box1, moments_red);
    let whole_green = volume(*box1, moments_green);
    let whole_blue = volume(*box1, moments_blue);
    let whole_weight = volume(*box1, weights);

    let (max_red, cut_red) = maximize(
        *box1,
        Direction::Red,
        box1.r0 + 1,
        box1.r1,
        whole_weight,
        whole_red,
        whole_green,
        whole_blue,
        weights,
        moments_red,
        moments_green,
        moments_blue,
    );
    let (max_green, cut_green) = maximize(
        *box1,
        Direction::Green,
        box1.g0 + 1,
        box1.g1,
        whole_weight,
        whole_red,
        whole_green,
        whole_blue,
        weights,
        moments_red,
        moments_green,
        moments_blue,
    );
    let (max_blue, cut_blue) = maximize(
        *box1,
        Direction::Blue,
        box1.b0 + 1,
        box1.b1,
        whole_weight,
        whole_red,
        whole_green,
        whole_blue,
        weights,
        moments_red,
        moments_green,
        moments_blue,
    );

    let direction = if max_red >= max_green && max_red >= max_blue {
        if cut_red.is_none() {
            return false;
        }
        Direction::Red
    } else if max_green >= max_red && max_green >= max_blue {
        if cut_green.is_none() {
            return false;
        }
        Direction::Green
    } else {
        if cut_blue.is_none() {
            return false;
        }
        Direction::Blue
    };

    box2.r1 = box1.r1;
    box2.g1 = box1.g1;
    box2.b1 = box1.b1;

    match direction {
        Direction::Red => {
            let cut = cut_red.expect("red direction has a cut");
            box2.r0 = cut;
            box1.r1 = cut;
            box2.g0 = box1.g0;
            box2.b0 = box1.b0;
        }
        Direction::Green => {
            let cut = cut_green.expect("green direction has a cut");
            box2.r0 = box1.r0;
            box2.g0 = cut;
            box1.g1 = cut;
            box2.b0 = box1.b0;
        }
        Direction::Blue => {
            let cut = cut_blue.expect("blue direction has a cut");
            box2.r0 = box1.r0;
            box2.g0 = box1.g0;
            box2.b0 = cut;
            box1.b1 = cut;
        }
    }

    box1.volume = (box1.r1 - box1.r0) * (box1.g1 - box1.g0) * (box1.b1 - box1.b0);
    box2.volume = (box2.r1 - box2.r0) * (box2.g1 - box2.g0) * (box2.b1 - box2.b0);
    true
}

#[allow(clippy::too_many_arguments)]
fn maximize(
    cube: Box3d,
    direction: Direction,
    first: usize,
    last: usize,
    whole_weight: i64,
    whole_red: i64,
    whole_green: i64,
    whole_blue: i64,
    weights: &IntArray,
    moments_red: &IntArray,
    moments_green: &IntArray,
    moments_blue: &IntArray,
) -> (f64, Option<usize>) {
    let bottom_red = bottom(cube, direction, moments_red);
    let bottom_green = bottom(cube, direction, moments_green);
    let bottom_blue = bottom(cube, direction, moments_blue);
    let bottom_weight = bottom(cube, direction, weights);
    let mut max = 0.0;
    let mut cut = None;

    for index in first..last {
        let mut half_red = bottom_red + top(cube, direction, index, moments_red);
        let mut half_green = bottom_green + top(cube, direction, index, moments_green);
        let mut half_blue = bottom_blue + top(cube, direction, index, moments_blue);
        let mut half_weight = bottom_weight + top(cube, direction, index, weights);
        if half_weight == 0 {
            continue;
        }

        let mut temp = (half_red as f64 * half_red as f64
            + half_green as f64 * half_green as f64
            + half_blue as f64 * half_blue as f64)
            / half_weight as f64;

        half_red = whole_red - half_red;
        half_green = whole_green - half_green;
        half_blue = whole_blue - half_blue;
        half_weight = whole_weight - half_weight;
        if half_weight == 0 {
            continue;
        }
        temp += (half_red as f64 * half_red as f64
            + half_green as f64 * half_green as f64
            + half_blue as f64 * half_blue as f64)
            / half_weight as f64;

        if temp > max {
            max = temp;
            cut = Some(index);
        }
    }

    (max, cut)
}

fn variance(
    cube: Box3d,
    weights: &IntArray,
    moments_red: &IntArray,
    moments_green: &IntArray,
    moments_blue: &IntArray,
    moments: &DoubleArray,
) -> f64 {
    let red = volume(cube, moments_red) as f64;
    let green = volume(cube, moments_green) as f64;
    let blue = volume(cube, moments_blue) as f64;
    let moment = moments[get_index(cube.r1, cube.g1, cube.b1)]
        - moments[get_index(cube.r1, cube.g1, cube.b0)]
        - moments[get_index(cube.r1, cube.g0, cube.b1)]
        + moments[get_index(cube.r1, cube.g0, cube.b0)]
        - moments[get_index(cube.r0, cube.g1, cube.b1)]
        + moments[get_index(cube.r0, cube.g1, cube.b0)]
        + moments[get_index(cube.r0, cube.g0, cube.b1)]
        - moments[get_index(cube.r0, cube.g0, cube.b0)];
    let hypotenuse = red * red + green * green + blue * blue;
    let cube_volume = volume(cube, weights) as f64;

    moment - hypotenuse / cube_volume
}

fn volume(cube: Box3d, moment: &IntArray) -> i64 {
    moment[get_index(cube.r1, cube.g1, cube.b1)]
        - moment[get_index(cube.r1, cube.g1, cube.b0)]
        - moment[get_index(cube.r1, cube.g0, cube.b1)]
        + moment[get_index(cube.r1, cube.g0, cube.b0)]
        - moment[get_index(cube.r0, cube.g1, cube.b1)]
        + moment[get_index(cube.r0, cube.g1, cube.b0)]
        + moment[get_index(cube.r0, cube.g0, cube.b1)]
        - moment[get_index(cube.r0, cube.g0, cube.b0)]
}

fn bottom(cube: Box3d, direction: Direction, moment: &IntArray) -> i64 {
    match direction {
        Direction::Red => {
            -moment[get_index(cube.r0, cube.g1, cube.b1)]
                + moment[get_index(cube.r0, cube.g1, cube.b0)]
                + moment[get_index(cube.r0, cube.g0, cube.b1)]
                - moment[get_index(cube.r0, cube.g0, cube.b0)]
        }
        Direction::Green => {
            -moment[get_index(cube.r1, cube.g0, cube.b1)]
                + moment[get_index(cube.r1, cube.g0, cube.b0)]
                + moment[get_index(cube.r0, cube.g0, cube.b1)]
                - moment[get_index(cube.r0, cube.g0, cube.b0)]
        }
        Direction::Blue => {
            -moment[get_index(cube.r1, cube.g1, cube.b0)]
                + moment[get_index(cube.r1, cube.g0, cube.b0)]
                + moment[get_index(cube.r0, cube.g1, cube.b0)]
                - moment[get_index(cube.r0, cube.g0, cube.b0)]
        }
    }
}

fn top(cube: Box3d, direction: Direction, position: usize, moment: &IntArray) -> i64 {
    match direction {
        Direction::Red => {
            moment[get_index(position, cube.g1, cube.b1)]
                - moment[get_index(position, cube.g1, cube.b0)]
                - moment[get_index(position, cube.g0, cube.b1)]
                + moment[get_index(position, cube.g0, cube.b0)]
        }
        Direction::Green => {
            moment[get_index(cube.r1, position, cube.b1)]
                - moment[get_index(cube.r1, position, cube.b0)]
                - moment[get_index(cube.r0, position, cube.b1)]
                + moment[get_index(cube.r0, position, cube.b0)]
        }
        Direction::Blue => {
            moment[get_index(cube.r1, cube.g1, position)]
                - moment[get_index(cube.r1, cube.g0, position)]
                - moment[get_index(cube.r0, cube.g1, position)]
                + moment[get_index(cube.r0, cube.g0, position)]
        }
    }
}

const fn get_index(red: usize, green: usize, blue: usize) -> usize {
    red * INDEX_COUNT * INDEX_COUNT + green * INDEX_COUNT + blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_primary_color_matches_cpp() {
        assert_eq!(
            quantize_wu(&[Argb::new(0xffff_0000)], 256),
            vec![Argb::new(0xffff_0000)]
        );
        assert_eq!(
            quantize_wu(&[Argb::new(0xff00_ff00)], 256),
            vec![Argb::new(0xff00_ff00)]
        );
        assert_eq!(
            quantize_wu(&[Argb::new(0xff00_00ff)], 256),
            vec![Argb::new(0xff00_00ff)]
        );
    }

    #[test]
    fn repeated_color_collapses_to_single_result() {
        let pixels = vec![Argb::new(0xff00_00ff); 5];
        assert_eq!(quantize_wu(&pixels, 256), vec![Argb::new(0xff00_00ff)]);
    }

    #[test]
    fn red_green_blue_matches_cpp_order() {
        let result = quantize_wu(
            &[
                Argb::new(0xffff_0000),
                Argb::new(0xff00_ff00),
                Argb::new(0xff00_00ff),
            ],
            256,
        );

        assert_eq!(
            result,
            vec![
                Argb::new(0xff00_00ff),
                Argb::new(0xffff_0000),
                Argb::new(0xff00_ff00),
            ]
        );
    }

    #[test]
    fn two_red_three_green_returns_two_colors() {
        let pixels = [
            Argb::new(0xffff_0000),
            Argb::new(0xffff_0000),
            Argb::new(0xffff_0000),
            Argb::new(0xff00_ff00),
            Argb::new(0xff00_ff00),
        ];

        assert_eq!(quantize_wu(&pixels, 256).len(), 2);
    }
}
