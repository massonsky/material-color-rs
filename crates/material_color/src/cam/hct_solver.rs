#![allow(clippy::many_single_char_names)]

use crate::argb::Argb;
use crate::cam::cam16::Cam;
use crate::cam::viewing_conditions::ViewingConditions;
use crate::utils::{
    PI, Vec3, argb_from_linrgb, int_from_lstar, matrix_multiply, sanitize_degrees_double, signum,
    y_from_lstar,
};

const SCALED_DISCOUNT_FROM_LINRGB: [[f64; 3]; 3] = [
    [
        0.001_200_833_568_784_504,
        0.002_389_694_492_170_889,
        0.000_279_574_288_586_112_4,
    ],
    [
        0.000_589_108_665_137_599_9,
        0.002_978_550_257_343_875_8,
        0.000_327_066_610_400_839_8,
    ],
    [
        0.000_101_466_924_916_405_72,
        0.000_536_421_435_918_669_4,
        0.003_297_940_177_071_207_6,
    ],
];

const LINRGB_FROM_SCALED_DISCOUNT: [[f64; 3]; 3] = [
    [
        1_373.219_870_959_423_1,
        -1_100.425_119_075_482_1,
        -7.278_681_089_101_213,
    ],
    [
        -271.815_969_077_903,
        559.658_046_594_073_3,
        -32.460_474_827_911_94,
    ],
    [
        1.962_289_959_966_566_6,
        -57.173_814_538_844_006,
        308.723_319_781_238_5,
    ],
];

const Y_FROM_LINRGB: [f64; 3] = [0.2126, 0.7152, 0.0722];

#[must_use]
pub fn solve_to_int(hue_degrees: f64, chroma: f64, lstar: f64) -> Argb {
    if chroma < 0.0001 || !(0.0001..=99.9999).contains(&lstar) {
        return int_from_lstar(lstar);
    }

    let hue_degrees = sanitize_degrees_double(hue_degrees);
    let hue_radians = hue_degrees / 180.0 * PI;
    let y = y_from_lstar(lstar);

    if let Some(exact_answer) = find_result_by_j(hue_radians, chroma, y) {
        return exact_answer;
    }

    argb_from_linrgb(bisect_to_limit(y, hue_radians))
}

#[must_use]
pub fn solve_to_cam(hue_degrees: f64, chroma: f64, lstar: f64) -> Cam {
    Cam::from_argb(solve_to_int(hue_degrees, chroma, lstar))
}

fn sanitize_radians(angle: f64) -> f64 {
    (angle + PI * 8.0) % (PI * 2.0)
}

fn true_delinearized(rgb_component: f64) -> f64 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.003_130_8 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };

    delinearized * 255.0
}

fn chromatic_adaptation(component: f64) -> f64 {
    let af = component.abs().powf(0.42);

    f64::from(signum(component)) * 400.0 * af / (af + 27.13)
}

fn hue_of(linrgb: Vec3) -> f64 {
    let scaled_discount = matrix_multiply(linrgb, &SCALED_DISCOUNT_FROM_LINRGB);
    let r_a = chromatic_adaptation(scaled_discount.a);
    let g_a = chromatic_adaptation(scaled_discount.b);
    let b_a = chromatic_adaptation(scaled_discount.c);
    let a = (11.0 * r_a - 12.0 * g_a + b_a) / 11.0;
    let b = (r_a + g_a - 2.0 * b_a) / 9.0;

    b.atan2(a)
}

fn are_in_cyclic_order(a: f64, b: f64, c: f64) -> bool {
    sanitize_radians(b - a) < sanitize_radians(c - a)
}

fn intercept(source: f64, mid: f64, target: f64) -> f64 {
    (mid - source) / (target - source)
}

fn lerp_point(source: Vec3, t: f64, target: Vec3) -> Vec3 {
    Vec3::new(
        source.a + (target.a - source.a) * t,
        source.b + (target.b - source.b) * t,
        source.c + (target.c - source.c) * t,
    )
}

fn axis(vector: Vec3, axis: usize) -> f64 {
    match axis {
        0 => vector.a,
        1 => vector.b,
        2 => vector.c,
        _ => -1.0,
    }
}

fn set_axis(vector: &mut Vec3, axis: usize, value: f64) {
    match axis {
        0 => vector.a = value,
        1 => vector.b = value,
        2 => vector.c = value,
        _ => {}
    }
}

fn set_coordinate(source: Vec3, coordinate: f64, target: Vec3, axis_index: usize) -> Vec3 {
    let t = intercept(
        axis(source, axis_index),
        coordinate,
        axis(target, axis_index),
    );

    lerp_point(source, t, target)
}

fn is_bounded(value: f64) -> bool {
    (0.0..=100.0).contains(&value)
}

fn nth_vertex(y: f64, n: usize) -> Option<Vec3> {
    let coord_a = if n % 4 <= 1 { 0.0 } else { 100.0 };
    let coord_b = if n % 2 == 0 { 0.0 } else { 100.0 };
    let mut vertex = Vec3::default();

    if n < 4 {
        vertex.b = coord_a;
        vertex.c = coord_b;
        vertex.a =
            (y - vertex.b * Y_FROM_LINRGB[1] - vertex.c * Y_FROM_LINRGB[2]) / Y_FROM_LINRGB[0];
    } else if n < 8 {
        vertex.c = coord_a;
        vertex.a = coord_b;
        vertex.b =
            (y - vertex.a * Y_FROM_LINRGB[0] - vertex.c * Y_FROM_LINRGB[2]) / Y_FROM_LINRGB[1];
    } else {
        vertex.a = coord_a;
        vertex.b = coord_b;
        vertex.c =
            (y - vertex.a * Y_FROM_LINRGB[0] - vertex.b * Y_FROM_LINRGB[1]) / Y_FROM_LINRGB[2];
    }

    (is_bounded(vertex.a) && is_bounded(vertex.b) && is_bounded(vertex.c)).then_some(vertex)
}

fn bisect_to_segment(y: f64, target_hue: f64) -> [Vec3; 2] {
    let mut left = Vec3::new(-1.0, -1.0, -1.0);
    let mut right = left;
    let mut left_hue = 0.0;
    let mut right_hue = 0.0;
    let mut initialized = false;
    let mut uncut = true;

    for n in 0..12 {
        let Some(mid) = nth_vertex(y, n) else {
            continue;
        };
        let mid_hue = hue_of(mid);

        if !initialized {
            left = mid;
            right = mid;
            left_hue = mid_hue;
            right_hue = mid_hue;
            initialized = true;
            continue;
        }

        if uncut || are_in_cyclic_order(left_hue, mid_hue, right_hue) {
            uncut = false;

            if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                right = mid;
                right_hue = mid_hue;
            } else {
                left = mid;
                left_hue = mid_hue;
            }
        }
    }

    [left, right]
}

fn midpoint(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(
        f64::midpoint(a.a, b.a),
        f64::midpoint(a.b, b.b),
        f64::midpoint(a.c, b.c),
    )
}

#[allow(clippy::cast_possible_truncation)]
fn critical_plane_below(value: f64) -> i32 {
    (value - 0.5).floor() as i32
}

#[allow(clippy::cast_possible_truncation)]
fn critical_plane_above(value: f64) -> i32 {
    (value - 0.5).ceil() as i32
}

fn critical_plane(index: i32) -> f64 {
    let normalized = (f64::from(index.clamp(0, 254)) + 0.5) / 255.0;

    if normalized <= 0.040_449_936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

#[allow(clippy::float_cmp)]
fn bisect_to_limit(y: f64, target_hue: f64) -> Vec3 {
    let [mut left, mut right] = bisect_to_segment(y, target_hue);
    let mut left_hue = hue_of(left);

    for axis_index in 0..3 {
        if axis(left, axis_index) != axis(right, axis_index) {
            let (mut l_plane, mut r_plane) = if axis(left, axis_index) < axis(right, axis_index) {
                (
                    critical_plane_below(true_delinearized(axis(left, axis_index))),
                    critical_plane_above(true_delinearized(axis(right, axis_index))),
                )
            } else {
                (
                    critical_plane_above(true_delinearized(axis(left, axis_index))),
                    critical_plane_below(true_delinearized(axis(right, axis_index))),
                )
            };

            for _ in 0..8 {
                if (r_plane - l_plane).abs() <= 1 {
                    break;
                }

                let m_plane = (l_plane + r_plane).div_euclid(2);
                let mid_plane_coordinate = critical_plane(m_plane);
                let mid = set_coordinate(left, mid_plane_coordinate, right, axis_index);
                let mid_hue = hue_of(mid);

                if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                    right = mid;
                    r_plane = m_plane;
                } else {
                    left = mid;
                    left_hue = mid_hue;
                    l_plane = m_plane;
                }
            }
        }
    }

    midpoint(left, right)
}

fn inverse_chromatic_adaptation(adapted: f64) -> f64 {
    let base = (27.13 * adapted.abs() / (400.0 - adapted.abs())).max(0.0);

    f64::from(signum(adapted)) * base.powf(1.0 / 0.42)
}

#[allow(clippy::too_many_lines)]
fn find_result_by_j(hue_radians: f64, chroma: f64, y: f64) -> Option<Argb> {
    let mut j = y.sqrt() * 11.0;
    let viewing_conditions = ViewingConditions::DEFAULT;
    let t_inner_coeff =
        1.0 / (1.64 - 0.29_f64.powf(viewing_conditions.background_y_to_white_point_y)).powf(0.73);
    let e_hue = 0.25 * ((hue_radians + 2.0).cos() + 3.8);
    let p1 = e_hue * (50_000.0 / 13.0) * viewing_conditions.n_c * viewing_conditions.ncb;
    let h_sin = hue_radians.sin();
    let h_cos = hue_radians.cos();

    for iteration_round in 0..5 {
        let j_normalized = j / 100.0;
        let alpha = if chroma == 0.0 || j == 0.0 {
            0.0
        } else {
            chroma / j_normalized.sqrt()
        };
        let t = (alpha * t_inner_coeff).powf(1.0 / 0.9);
        let ac = viewing_conditions.aw
            * j_normalized.powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p2 = ac / viewing_conditions.nbb;
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;
        let scaled = Vec3::new(
            inverse_chromatic_adaptation(r_a),
            inverse_chromatic_adaptation(g_a),
            inverse_chromatic_adaptation(b_a),
        );
        let mut linrgb = matrix_multiply(scaled, &LINRGB_FROM_SCALED_DISCOUNT);

        if linrgb.a < 0.0 || linrgb.b < 0.0 || linrgb.c < 0.0 {
            return None;
        }

        let fnj =
            Y_FROM_LINRGB[0] * linrgb.a + Y_FROM_LINRGB[1] * linrgb.b + Y_FROM_LINRGB[2] * linrgb.c;

        if fnj <= 0.0 {
            return None;
        }

        if iteration_round == 4 || (fnj - y).abs() < 0.002 {
            if linrgb.a > 100.01 || linrgb.b > 100.01 || linrgb.c > 100.01 {
                return None;
            }

            for axis_index in 0..3 {
                let value = axis(linrgb, axis_index).max(0.0);
                set_axis(&mut linrgb, axis_index, value);
            }

            return Some(argb_from_linrgb(linrgb));
        }

        j -= (fnj - y) * j / (2.0 * fnj);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::lstar_from_argb;

    #[test]
    fn solve_to_int_round_trips_cpp_red_case() {
        let color = Argb::new(0xfffe_0315);
        let cam = Cam::from_argb(color);
        let tone = lstar_from_argb(color);

        assert_eq!(solve_to_int(cam.hue, cam.chroma, tone), color);
    }

    #[test]
    fn solve_to_int_round_trips_cpp_green_case() {
        let color = Argb::new(0xff15_fe03);
        let cam = Cam::from_argb(color);
        let tone = lstar_from_argb(color);

        assert_eq!(solve_to_int(cam.hue, cam.chroma, tone), color);
    }

    #[test]
    fn solve_to_int_round_trips_cpp_blue_case() {
        let color = Argb::new(0xff03_15fe);
        let cam = Cam::from_argb(color);
        let tone = lstar_from_argb(color);

        assert_eq!(solve_to_int(cam.hue, cam.chroma, tone), color);
    }

    #[test]
    fn solve_to_int_round_trips_representative_grid() {
        for red in [0x00, 0x03, 0x15, 0x40, 0x80, 0xc0, 0xfe, 0xff] {
            for green in [0x00, 0x03, 0x15, 0x40, 0x80, 0xc0, 0xfe, 0xff] {
                for blue in [0x00, 0x03, 0x15, 0x40, 0x80, 0xc0, 0xfe, 0xff] {
                    let color = crate::argb::argb_from_rgb(red, green, blue);
                    let cam = Cam::from_argb(color);
                    let tone = lstar_from_argb(color);

                    assert_eq!(
                        solve_to_int(cam.hue, cam.chroma, tone),
                        color,
                        "failed for #{:08x}",
                        color.to_u32(),
                    );
                }
            }
        }
    }
}
