use crate::argb::Argb;
use crate::cam::Hct;
use crate::utils::{PI, WHITE_POINT_D65, sanitize_degrees_double, sanitize_degrees_int};

#[derive(Clone, Debug)]
pub(crate) struct TemperatureCache {
    input: Hct,
    complement: Option<Hct>,
    hcts_by_hue: Option<Vec<Hct>>,
    hcts_by_temp: Option<Vec<Hct>>,
}

impl TemperatureCache {
    #[must_use]
    pub(crate) const fn new(input: Hct) -> Self {
        Self {
            input,
            complement: None,
            hcts_by_hue: None,
            hcts_by_temp: None,
        }
    }

    pub(crate) fn complement(&mut self) -> Hct {
        if let Some(complement) = self.complement {
            return complement;
        }

        let coldest = self.coldest();
        let coldest_hue = coldest.hue();
        let coldest_temp = Self::raw_temperature(coldest);
        let warmest = self.warmest();
        let warmest_hue = warmest.hue();
        let warmest_temp = Self::raw_temperature(warmest);
        let range = warmest_temp - coldest_temp;
        let start_hue_is_coldest_to_warmest =
            Self::is_between(self.input.hue(), coldest_hue, warmest_hue);
        let start_hue = if start_hue_is_coldest_to_warmest {
            warmest_hue
        } else {
            coldest_hue
        };
        let end_hue = if start_hue_is_coldest_to_warmest {
            coldest_hue
        } else {
            warmest_hue
        };
        let complement_relative_temp = 1.0 - self.relative_temperature(self.input);
        let hcts_by_hue = self.hcts_by_hue();
        let mut smallest_error = 1000.0;
        let mut answer = hcts_by_hue[round_to_hue_index(self.input.hue())];

        for hue_addend in 0..=360 {
            let hue = sanitize_degrees_double(start_hue + f64::from(hue_addend));
            if !Self::is_between(hue, start_hue, end_hue) {
                continue;
            }

            let possible_answer = hcts_by_hue[round_to_hue_index(hue)];
            let relative_temp = if range == 0.0 {
                0.5
            } else {
                (Self::raw_temperature(possible_answer) - coldest_temp) / range
            };
            let error = (complement_relative_temp - relative_temp).abs();
            if error < smallest_error {
                smallest_error = error;
                answer = possible_answer;
            }
        }

        self.complement = Some(answer);
        answer
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::similar_names
    )]
    pub(crate) fn analogous_colors(&mut self, count: usize, divisions: usize) -> Vec<Hct> {
        let start_hue = round_to_hue_index(self.input.hue());
        let hcts_by_hue = self.hcts_by_hue();
        let start_hct = hcts_by_hue[start_hue];
        let mut last_temp = self.relative_temperature(start_hct);
        let mut all_colors = vec![start_hct];
        let mut absolute_total_temp_delta = 0.0;

        for index in 0..360 {
            let hue = sanitize_degrees_int(start_hue as i32 + index);
            let hct = hcts_by_hue[hue as usize];
            let temp = self.relative_temperature(hct);
            absolute_total_temp_delta += (temp - last_temp).abs();
            last_temp = temp;
        }

        let mut hue_addend = 1;
        let temp_step = absolute_total_temp_delta / divisions as f64;
        let mut total_temp_delta = 0.0;
        last_temp = self.relative_temperature(start_hct);
        while all_colors.len() < divisions {
            let hue = sanitize_degrees_int(start_hue as i32 + hue_addend);
            let hct = hcts_by_hue[hue as usize];
            let temp = self.relative_temperature(hct);
            total_temp_delta += (temp - last_temp).abs();

            let mut desired_total_temp_delta_for_index = all_colors.len() as f64 * temp_step;
            let mut index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
            let mut index_addend = 1;
            while index_satisfied && all_colors.len() < divisions {
                all_colors.push(hct);
                desired_total_temp_delta_for_index =
                    (all_colors.len() + index_addend) as f64 * temp_step;
                index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
                index_addend += 1;
            }
            last_temp = temp;
            hue_addend += 1;

            if hue_addend > 360 {
                while all_colors.len() < divisions {
                    all_colors.push(hct);
                }
                break;
            }
        }

        let mut answers = vec![self.input];
        let ccw_count = ((count as f64 - 1.0) / 2.0).floor() as isize;
        for index in 1..=ccw_count {
            let mut all_colors_index = -index;
            while all_colors_index < 0 {
                all_colors_index += all_colors.len() as isize;
            }
            let all_colors_index = all_colors_index as usize % all_colors.len();
            answers.insert(0, all_colors[all_colors_index]);
        }

        let cw_count = count as isize - ccw_count - 1;
        for index in 1..=cw_count {
            let all_colors_index = index as usize % all_colors.len();
            answers.push(all_colors[all_colors_index]);
        }

        answers
    }

    pub(crate) fn relative_temperature(&mut self, hct: Hct) -> f64 {
        let coldest_temp = Self::raw_temperature(self.coldest());
        let warmest_temp = Self::raw_temperature(self.warmest());
        let range = warmest_temp - coldest_temp;

        if range == 0.0 {
            0.5
        } else {
            (Self::raw_temperature(hct) - coldest_temp) / range
        }
    }

    #[must_use]
    pub(crate) fn raw_temperature(color: Hct) -> f64 {
        let lab = lab_from_argb(color.to_argb());
        let hue = sanitize_degrees_double(lab.b.atan2(lab.a) * 180.0 / PI);
        let chroma = lab.a.hypot(lab.b);

        -0.5 + 0.02 * chroma.powf(1.07) * (sanitize_degrees_double(hue - 50.0) * PI / 180.0).cos()
    }

    fn coldest(&mut self) -> Hct {
        self.hcts_by_temp()[0]
    }

    fn warmest(&mut self) -> Hct {
        let hcts_by_temp = self.hcts_by_temp();
        hcts_by_temp[hcts_by_temp.len() - 1]
    }

    fn hcts_by_hue(&mut self) -> Vec<Hct> {
        if let Some(hcts_by_hue) = &self.hcts_by_hue {
            return hcts_by_hue.clone();
        }

        let hcts_by_hue = (0..=360)
            .map(|hue| Hct::new(f64::from(hue), self.input.chroma(), self.input.tone()))
            .collect::<Vec<_>>();
        self.hcts_by_hue = Some(hcts_by_hue.clone());
        hcts_by_hue
    }

    fn hcts_by_temp(&mut self) -> Vec<Hct> {
        if let Some(hcts_by_temp) = &self.hcts_by_temp {
            return hcts_by_temp.clone();
        }

        let mut hcts = self.hcts_by_hue();
        hcts.push(self.input);
        hcts.sort_by(|left, right| {
            Self::raw_temperature(*left).total_cmp(&Self::raw_temperature(*right))
        });
        self.hcts_by_temp = Some(hcts.clone());
        hcts
    }

    const fn is_between(angle: f64, a: f64, b: f64) -> bool {
        if a < b {
            a <= angle && angle <= b
        } else {
            a <= angle || angle <= b
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Lab {
    l: f64,
    a: f64,
    b: f64,
}

#[allow(clippy::similar_names)]
fn lab_from_argb(argb: Argb) -> Lab {
    let red_l = crate::utils::linearized(argb.red());
    let green_l = crate::utils::linearized(argb.green());
    let blue_l = crate::utils::linearized(argb.blue());
    let x = 0.412_338_95 * red_l + 0.357_620_64 * green_l + 0.180_510_42 * blue_l;
    let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
    let z = 0.019_321_41 * red_l + 0.119_163_82 * green_l + 0.950_344_78 * blue_l;
    let fy = lab_f(y / WHITE_POINT_D65[1]);
    let fx = lab_f(x / WHITE_POINT_D65[0]);
    let fz = lab_f(z / WHITE_POINT_D65[2]);

    Lab {
        l: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

fn lab_f(normalized: f64) -> f64 {
    const E: f64 = 216.0 / 24_389.0;
    const KAPPA: f64 = 24_389.0 / 27.0;

    if normalized > E {
        normalized.powf(1.0 / 3.0)
    } else {
        (KAPPA * normalized + 16.0) / 116.0
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn round_to_hue_index(hue: f64) -> usize {
    hue.round().clamp(0.0, 360.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hex_from_argb;

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}",
        );
    }

    #[test]
    fn raw_temperature_matches_cpp() {
        assert_near(
            TemperatureCache::raw_temperature(Hct::from_argb(Argb::new(0xff00_00ff))),
            -1.393,
            0.001,
        );
        assert_near(
            TemperatureCache::raw_temperature(Hct::from_argb(Argb::new(0xffff_0000))),
            2.351,
            0.001,
        );
        assert_near(
            TemperatureCache::raw_temperature(Hct::from_argb(Argb::new(0xff00_ff00))),
            -0.267,
            0.001,
        );
        assert_near(
            TemperatureCache::raw_temperature(Hct::from_argb(Argb::new(0xffff_ffff))),
            -0.5,
            0.001,
        );
        assert_near(
            TemperatureCache::raw_temperature(Hct::from_argb(Argb::new(0xff00_0000))),
            -0.5,
            0.001,
        );
    }

    #[test]
    fn complement_matches_cpp() {
        for (source, expected) in [
            (0xff00_00ff, "ff9d0002"),
            (0xffff_0000, "ff007bfc"),
            (0xff00_ff00, "ffffd2c9"),
            (0xffff_ffff, "ffffffff"),
            (0xff00_0000, "ff000000"),
        ] {
            let mut cache = TemperatureCache::new(Hct::from_argb(Argb::new(source)));

            assert_eq!(hex_from_argb(cache.complement().to_argb()), expected);
        }
    }

    #[test]
    fn analogous_matches_cpp() {
        let mut blue_cache = TemperatureCache::new(Hct::from_argb(Argb::new(0xff00_00ff)));
        let blue = blue_cache.analogous_colors(5, 12);
        assert_eq!(hex_from_argb(blue[0].to_argb()), "ff00590c");
        assert_eq!(hex_from_argb(blue[1].to_argb()), "ff00564e");
        assert_eq!(hex_from_argb(blue[2].to_argb()), "ff0000ff");
        assert_eq!(hex_from_argb(blue[3].to_argb()), "ff6700cc");
        assert_eq!(hex_from_argb(blue[4].to_argb()), "ff81009f");

        let mut white_cache = TemperatureCache::new(Hct::from_argb(Argb::new(0xffff_ffff)));
        let white = white_cache.analogous_colors(5, 12);
        assert!(
            white
                .iter()
                .all(|hct| hct.to_argb() == Argb::new(0xffff_ffff))
        );
    }
}
