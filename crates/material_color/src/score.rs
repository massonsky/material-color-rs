use std::collections::BTreeMap;

use crate::argb::Argb;
use crate::cam::Hct;
use crate::utils::{diff_degrees, sanitize_degrees_int};

const TARGET_CHROMA: f64 = 48.0;
const WEIGHT_PROPORTION: f64 = 0.7;
const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
const WEIGHT_CHROMA_BELOW: f64 = 0.1;
const CUTOFF_CHROMA: f64 = 5.0;
const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScoreOptions {
    pub desired: usize,
    pub fallback_color_argb: Argb,
    pub filter: bool,
}

impl Default for ScoreOptions {
    fn default() -> Self {
        Self {
            desired: 4,
            fallback_color_argb: Argb::new(0xff42_85f4),
            filter: true,
        }
    }
}

#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn ranked_suggestions(
    argb_to_population: &BTreeMap<Argb, u32>,
    options: ScoreOptions,
) -> Vec<Argb> {
    let mut colors_hct = Vec::with_capacity(argb_to_population.len());
    let mut hue_population = vec![0_u32; 360];
    let mut population_sum = 0.0;

    for (&argb, &population) in argb_to_population {
        let hct = Hct::from_argb(argb);
        colors_hct.push(hct);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let hue = hct.hue().floor() as usize;
        hue_population[hue] += population;
        population_sum += f64::from(population);
    }

    let mut hue_excited_proportions = vec![0.0; 360];
    if population_sum > 0.0 {
        for (hue, population) in hue_population.iter().enumerate() {
            let proportion = f64::from(*population) / population_sum;
            #[allow(clippy::cast_possible_wrap)]
            for index in hue as i32 - 14..hue as i32 + 16 {
                let neighbor_hue = sanitize_degrees_int(index);
                #[allow(clippy::cast_sign_loss)]
                {
                    hue_excited_proportions[neighbor_hue as usize] += proportion;
                }
            }
        }
    }

    let mut scored_hcts = Vec::<(Hct, f64)>::new();
    for hct in colors_hct {
        #[allow(clippy::cast_possible_truncation)]
        let hue = sanitize_degrees_int(hct.hue().round() as i32);
        #[allow(clippy::cast_sign_loss)]
        let proportion = hue_excited_proportions[hue as usize];
        if options.filter
            && (hct.chroma() < CUTOFF_CHROMA || proportion <= CUTOFF_EXCITED_PROPORTION)
        {
            continue;
        }

        let proportion_score = proportion * 100.0 * WEIGHT_PROPORTION;
        let chroma_weight = if hct.chroma() < TARGET_CHROMA {
            WEIGHT_CHROMA_BELOW
        } else {
            WEIGHT_CHROMA_ABOVE
        };
        let chroma_score = (hct.chroma() - TARGET_CHROMA) * chroma_weight;
        scored_hcts.push((hct, proportion_score + chroma_score));
    }
    scored_hcts.sort_by(|left, right| right.1.total_cmp(&left.1));

    let mut chosen_colors = Vec::new();
    for difference_degrees in (15..=90).rev() {
        chosen_colors.clear();
        for &(hct, _) in &scored_hcts {
            let duplicate_hue = chosen_colors.iter().any(|chosen_hct: &Hct| {
                diff_degrees(hct.hue(), chosen_hct.hue()) < f64::from(difference_degrees)
            });
            if !duplicate_hue {
                chosen_colors.push(hct);
                if chosen_colors.len() >= options.desired {
                    break;
                }
            }
        }
        if chosen_colors.len() >= options.desired {
            break;
        }
    }

    if chosen_colors.is_empty() {
        vec![options.fallback_color_argb]
    } else {
        chosen_colors.into_iter().map(Hct::to_argb).collect()
    }
}

#[must_use]
pub fn ranked_suggestions_default(argb_to_population: &BTreeMap<Argb, u32>) -> Vec<Argb> {
    ranked_suggestions(argb_to_population, ScoreOptions::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(entries: &[(u32, u32)]) -> BTreeMap<Argb, u32> {
        entries
            .iter()
            .map(|&(argb, population)| (Argb::new(argb), population))
            .collect()
    }

    fn assert_ranked(entries: &[(u32, u32)], options: ScoreOptions, expected: &[u32]) {
        let actual = ranked_suggestions(&map(entries), options)
            .into_iter()
            .map(Argb::to_u32)
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn prioritizes_chroma() {
        assert_ranked(
            &[(0xff00_0000, 1), (0xffff_ffff, 1), (0xff00_00ff, 1)],
            ScoreOptions {
                desired: 4,
                ..ScoreOptions::default()
            },
            &[0xff00_00ff],
        );
    }

    #[test]
    fn prioritizes_chroma_when_proportions_equal() {
        assert_ranked(
            &[(0xffff_0000, 1), (0xff00_ff00, 1), (0xff00_00ff, 1)],
            ScoreOptions {
                desired: 4,
                ..ScoreOptions::default()
            },
            &[0xffff_0000, 0xff00_ff00, 0xff00_00ff],
        );
    }

    #[test]
    fn generates_google_blue_when_no_colors_available() {
        assert_ranked(
            &[(0xff00_0000, 1)],
            ScoreOptions {
                desired: 4,
                ..ScoreOptions::default()
            },
            &[0xff42_85f4],
        );
    }

    #[test]
    fn dedupes_nearby_hues() {
        assert_ranked(
            &[(0xff00_8772, 1), (0xff31_8477, 1)],
            ScoreOptions {
                desired: 4,
                ..ScoreOptions::default()
            },
            &[0xff00_8772],
        );
    }

    #[test]
    fn maximizes_hue_distance() {
        assert_ranked(
            &[(0xff00_8772, 1), (0xff00_8587, 1), (0xff00_7ebc, 1)],
            ScoreOptions {
                desired: 2,
                ..ScoreOptions::default()
            },
            &[0xff00_7ebc, 0xff00_8772],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn generated_scenarios_match_cpp() {
        assert_ranked(
            &[(0xff7e_a16d, 67), (0xffd8_ccae, 67), (0xff83_5c0d, 49)],
            ScoreOptions {
                desired: 3,
                fallback_color_argb: Argb::new(0xff8d_3819),
                filter: false,
            },
            &[0xff7e_a16d, 0xffd8_ccae, 0xff83_5c0d],
        );
        assert_ranked(
            &[
                (0xffd3_3881, 14),
                (0xff32_05cc, 77),
                (0xff0b_48cf, 36),
                (0xffa0_8f5d, 81),
            ],
            ScoreOptions {
                desired: 4,
                fallback_color_argb: Argb::new(0xff7d_772b),
                filter: true,
            },
            &[0xff32_05cc, 0xffa0_8f5d, 0xffd3_3881],
        );
        assert_ranked(
            &[
                (0xffbe_94a6, 23),
                (0xffc3_3fd7, 42),
                (0xff89_9f36, 90),
                (0xff94_c574, 82),
            ],
            ScoreOptions {
                desired: 3,
                fallback_color_argb: Argb::new(0xffaa_79a4),
                filter: true,
            },
            &[0xff94_c574, 0xffc3_3fd7, 0xffbe_94a6],
        );
        assert_ranked(
            &[
                (0xffdf_241c, 85),
                (0xff68_5859, 44),
                (0xffd0_6d5f, 34),
                (0xff56_1c54, 27),
                (0xff71_3090, 88),
            ],
            ScoreOptions {
                desired: 5,
                fallback_color_argb: Argb::new(0xff58_c19c),
                filter: false,
            },
            &[0xffdf_241c, 0xff56_1c54],
        );
        assert_ranked(
            &[
                (0xffbe_66f8, 41),
                (0xff4b_bda9, 88),
                (0xff80_f6f9, 44),
                (0xffab_8017, 43),
                (0xffe8_9307, 65),
            ],
            ScoreOptions {
                desired: 3,
                fallback_color_argb: Argb::new(0xff91_6691),
                filter: false,
            },
            &[0xffab_8017, 0xff4b_bda9, 0xffbe_66f8],
        );
        assert_ranked(
            &[
                (0xff18_ea8f, 93),
                (0xff32_7593, 18),
                (0xff06_6a18, 53),
                (0xfffa_8a23, 74),
                (0xff04_ca1f, 62),
            ],
            ScoreOptions {
                desired: 2,
                fallback_color_argb: Argb::new(0xff4c_377a),
                filter: false,
            },
            &[0xff18_ea8f, 0xfffa_8a23],
        );
        assert_ranked(
            &[
                (0xff2e_05ed, 23),
                (0xff15_3e55, 90),
                (0xff9a_b220, 23),
                (0xff15_3379, 66),
                (0xff68_bcc3, 81),
            ],
            ScoreOptions {
                desired: 2,
                fallback_color_argb: Argb::new(0xfff5_88dc),
                filter: true,
            },
            &[0xff2e_05ed, 0xff9a_b220],
        );
        assert_ranked(
            &[
                (0xff81_6ec5, 24),
                (0xff6d_cb94, 19),
                (0xff3c_ae91, 98),
                (0xff5b_542f, 25),
            ],
            ScoreOptions {
                desired: 1,
                fallback_color_argb: Argb::new(0xff84_b0fd),
                filter: false,
            },
            &[0xff3c_ae91],
        );
        assert_ranked(
            &[
                (0xff20_6f86, 52),
                (0xff4a_620d, 96),
                (0xfff5_1401, 85),
                (0xff2b_8ebf, 3),
                (0xff27_7766, 59),
            ],
            ScoreOptions {
                desired: 3,
                fallback_color_argb: Argb::new(0xff02_b415),
                filter: true,
            },
            &[0xfff5_1401, 0xff4a_620d, 0xff2b_8ebf],
        );
        assert_ranked(
            &[
                (0xff8b_1d99, 54),
                (0xff27_effe, 43),
                (0xff6f_558d, 2),
                (0xff77_fdf2, 78),
            ],
            ScoreOptions {
                desired: 4,
                fallback_color_argb: Argb::new(0xff5e_7a10),
                filter: true,
            },
            &[0xff27_effe, 0xff8b_1d99, 0xff6f_558d],
        );
    }
}
