use crate::argb::Argb;
use crate::cam::Hct;
use crate::contrast::ratio_of_tones;
use crate::dislike::fix_if_disliked;
use crate::palettes::TonalPalette;
use crate::palettes::core::rotated_hue;
use crate::temperature::TemperatureCache;
use crate::utils::sanitize_degrees_double;

use super::color::{
    ContrastCurve, ToneDeltaPair, TonePolarity, dual_background_tone, foreground_tone,
};
use super::roles::DynamicColorRole;
use super::variant::Variant;

type Role = DynamicColorRole;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicScheme {
    source_color_hct: Hct,
    variant: Variant,
    is_dark: bool,
    contrast_level: f64,
    primary_palette: TonalPalette,
    secondary_palette: TonalPalette,
    tertiary_palette: TonalPalette,
    neutral_palette: TonalPalette,
    neutral_variant_palette: TonalPalette,
    error_palette: TonalPalette,
}

impl DynamicScheme {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_color_hct: Hct,
        variant: Variant,
        is_dark: bool,
        contrast_level: f64,
        primary_palette: TonalPalette,
        secondary_palette: TonalPalette,
        tertiary_palette: TonalPalette,
        neutral_palette: TonalPalette,
        neutral_variant_palette: TonalPalette,
        error_palette: Option<TonalPalette>,
    ) -> Self {
        Self {
            source_color_hct,
            variant,
            is_dark,
            contrast_level,
            primary_palette,
            secondary_palette,
            tertiary_palette,
            neutral_palette,
            neutral_variant_palette,
            error_palette: error_palette
                .unwrap_or_else(|| TonalPalette::from_hue_and_chroma(25.0, 84.0)),
        }
    }

    #[must_use]
    pub fn from_argb(
        source_color: Argb,
        variant: Variant,
        is_dark: bool,
        contrast_level: f64,
    ) -> Self {
        Self::from_hct(
            Hct::from_argb(source_color),
            variant,
            is_dark,
            contrast_level,
        )
    }

    #[must_use]
    pub fn from_hct(
        source_color_hct: Hct,
        variant: Variant,
        is_dark: bool,
        contrast_level: f64,
    ) -> Self {
        match variant {
            Variant::Monochrome => Self::monochrome(source_color_hct, is_dark, contrast_level),
            Variant::Neutral => Self::neutral(source_color_hct, is_dark, contrast_level),
            Variant::TonalSpot => Self::tonal_spot(source_color_hct, is_dark, contrast_level),
            Variant::Vibrant => Self::vibrant(source_color_hct, is_dark, contrast_level),
            Variant::Expressive => Self::expressive(source_color_hct, is_dark, contrast_level),
            Variant::Fidelity => Self::fidelity(source_color_hct, is_dark, contrast_level),
            Variant::Content => Self::content(source_color_hct, is_dark, contrast_level),
            Variant::Rainbow => Self::rainbow(source_color_hct, is_dark, contrast_level),
            Variant::FruitSalad => Self::fruit_salad(source_color_hct, is_dark, contrast_level),
        }
    }

    #[must_use]
    pub fn tonal_spot(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        Self::new(
            source_color_hct,
            Variant::TonalSpot,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, 36.0),
            TonalPalette::from_hue_and_chroma(hue, 16.0),
            TonalPalette::from_hue_and_chroma(sanitize_degrees_double(hue + 60.0), 24.0),
            TonalPalette::from_hue_and_chroma(hue, 6.0),
            TonalPalette::from_hue_and_chroma(hue, 8.0),
            None,
        )
    }

    #[must_use]
    pub fn vibrant(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        const HUES: [f64; 9] = [0.0, 41.0, 61.0, 101.0, 131.0, 181.0, 251.0, 301.0, 360.0];
        const SECONDARY_ROTATIONS: [f64; 9] =
            [18.0, 15.0, 10.0, 12.0, 15.0, 18.0, 15.0, 12.0, 12.0];
        const TERTIARY_ROTATIONS: [f64; 9] = [35.0, 30.0, 20.0, 25.0, 30.0, 35.0, 30.0, 25.0, 25.0];

        Self::new(
            source_color_hct,
            Variant::Vibrant,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 200.0),
            TonalPalette::from_hue_and_chroma(
                rotated_hue(source_color_hct, &HUES, &SECONDARY_ROTATIONS),
                24.0,
            ),
            TonalPalette::from_hue_and_chroma(
                rotated_hue(source_color_hct, &HUES, &TERTIARY_ROTATIONS),
                32.0,
            ),
            TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 10.0),
            TonalPalette::from_hue_and_chroma(source_color_hct.hue(), 12.0),
            None,
        )
    }

    #[must_use]
    pub fn expressive(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        const HUES: [f64; 9] = [0.0, 21.0, 51.0, 121.0, 151.0, 191.0, 271.0, 321.0, 360.0];
        const SECONDARY_ROTATIONS: [f64; 9] =
            [45.0, 95.0, 45.0, 20.0, 45.0, 90.0, 45.0, 45.0, 45.0];
        const TERTIARY_ROTATIONS: [f64; 9] =
            [120.0, 120.0, 20.0, 45.0, 20.0, 15.0, 20.0, 120.0, 120.0];

        Self::new(
            source_color_hct,
            Variant::Expressive,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(source_color_hct.hue() + 240.0, 40.0),
            TonalPalette::from_hue_and_chroma(
                rotated_hue(source_color_hct, &HUES, &SECONDARY_ROTATIONS),
                24.0,
            ),
            TonalPalette::from_hue_and_chroma(
                rotated_hue(source_color_hct, &HUES, &TERTIARY_ROTATIONS),
                32.0,
            ),
            TonalPalette::from_hue_and_chroma(source_color_hct.hue() + 15.0, 8.0),
            TonalPalette::from_hue_and_chroma(source_color_hct.hue() + 15.0, 12.0),
            None,
        )
    }

    #[must_use]
    pub fn neutral(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        Self::new(
            source_color_hct,
            Variant::Neutral,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, 12.0),
            TonalPalette::from_hue_and_chroma(hue, 8.0),
            TonalPalette::from_hue_and_chroma(hue, 16.0),
            TonalPalette::from_hue_and_chroma(hue, 2.0),
            TonalPalette::from_hue_and_chroma(hue, 2.0),
            None,
        )
    }

    #[must_use]
    pub fn monochrome(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        Self::new(
            source_color_hct,
            Variant::Monochrome,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            None,
        )
    }

    #[must_use]
    pub fn fidelity(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        let chroma = source_color_hct.chroma();
        let mut temperature_cache = TemperatureCache::new(source_color_hct);
        let complement = fix_if_disliked(temperature_cache.complement());

        Self::new(
            source_color_hct,
            Variant::Fidelity,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, chroma),
            TonalPalette::from_hue_and_chroma(hue, (chroma - 32.0).max(chroma * 0.5)),
            TonalPalette::from_hct(complement),
            TonalPalette::from_hue_and_chroma(hue, chroma / 8.0),
            TonalPalette::from_hue_and_chroma(hue, chroma / 8.0 + 4.0),
            None,
        )
    }

    #[must_use]
    pub fn content(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        let chroma = source_color_hct.chroma();
        let mut temperature_cache = TemperatureCache::new(source_color_hct);
        let analogous = temperature_cache.analogous_colors(3, 6);
        let tertiary = fix_if_disliked(analogous[2]);

        Self::new(
            source_color_hct,
            Variant::Content,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, chroma),
            TonalPalette::from_hue_and_chroma(hue, (chroma - 32.0).max(chroma * 0.5)),
            TonalPalette::from_hct(tertiary),
            TonalPalette::from_hue_and_chroma(hue, chroma / 8.0),
            TonalPalette::from_hue_and_chroma(hue, chroma / 8.0 + 4.0),
            None,
        )
    }

    #[must_use]
    pub fn rainbow(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        Self::new(
            source_color_hct,
            Variant::Rainbow,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(hue, 48.0),
            TonalPalette::from_hue_and_chroma(hue, 16.0),
            TonalPalette::from_hue_and_chroma(sanitize_degrees_double(hue + 60.0), 24.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            TonalPalette::from_hue_and_chroma(hue, 0.0),
            None,
        )
    }

    #[must_use]
    pub fn fruit_salad(source_color_hct: Hct, is_dark: bool, contrast_level: f64) -> Self {
        let hue = source_color_hct.hue();
        let shifted_hue = sanitize_degrees_double(hue - 50.0);
        Self::new(
            source_color_hct,
            Variant::FruitSalad,
            is_dark,
            contrast_level,
            TonalPalette::from_hue_and_chroma(shifted_hue, 48.0),
            TonalPalette::from_hue_and_chroma(shifted_hue, 36.0),
            TonalPalette::from_hue_and_chroma(hue, 36.0),
            TonalPalette::from_hue_and_chroma(hue, 10.0),
            TonalPalette::from_hue_and_chroma(hue, 16.0),
            None,
        )
    }

    #[must_use]
    pub const fn source_color_hct(&self) -> Hct {
        self.source_color_hct
    }

    #[must_use]
    pub const fn source_color_argb(&self) -> Argb {
        self.source_color_hct.to_argb()
    }

    #[must_use]
    pub const fn variant(&self) -> Variant {
        self.variant
    }

    #[must_use]
    pub const fn is_dark(&self) -> bool {
        self.is_dark
    }

    #[must_use]
    pub const fn contrast_level(&self) -> f64 {
        self.contrast_level
    }

    #[must_use]
    pub const fn primary_palette(&self) -> TonalPalette {
        self.primary_palette
    }

    #[must_use]
    pub const fn secondary_palette(&self) -> TonalPalette {
        self.secondary_palette
    }

    #[must_use]
    pub const fn tertiary_palette(&self) -> TonalPalette {
        self.tertiary_palette
    }

    #[must_use]
    pub const fn neutral_palette(&self) -> TonalPalette {
        self.neutral_palette
    }

    #[must_use]
    pub const fn neutral_variant_palette(&self) -> TonalPalette {
        self.neutral_variant_palette
    }

    #[must_use]
    pub const fn error_palette(&self) -> TonalPalette {
        self.error_palette
    }

    #[must_use]
    pub fn color(&self, role: Role) -> Argb {
        self.palette(role).get(self.tone(role))
    }

    #[must_use]
    pub fn hct(&self, role: Role) -> Hct {
        Hct::from_argb(self.color(role))
    }

    #[must_use]
    pub fn tone(&self, role: Role) -> f64 {
        self.dynamic_tone(role)
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn palette(&self, role: Role) -> TonalPalette {
        match role {
            Role::PrimaryPaletteKeyColor
            | Role::SurfaceTint
            | Role::Primary
            | Role::OnPrimary
            | Role::PrimaryContainer
            | Role::OnPrimaryContainer
            | Role::InversePrimary
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::OnPrimaryFixed
            | Role::OnPrimaryFixedVariant => self.primary_palette,
            Role::SecondaryPaletteKeyColor
            | Role::Secondary
            | Role::OnSecondary
            | Role::SecondaryContainer
            | Role::OnSecondaryContainer
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::OnSecondaryFixed
            | Role::OnSecondaryFixedVariant => self.secondary_palette,
            Role::TertiaryPaletteKeyColor
            | Role::Tertiary
            | Role::OnTertiary
            | Role::TertiaryContainer
            | Role::OnTertiaryContainer
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim
            | Role::OnTertiaryFixed
            | Role::OnTertiaryFixedVariant => self.tertiary_palette,
            Role::NeutralVariantPaletteKeyColor
            | Role::SurfaceVariant
            | Role::OnSurfaceVariant
            | Role::Outline
            | Role::OutlineVariant => self.neutral_variant_palette,
            Role::Error | Role::OnError | Role::ErrorContainer | Role::OnErrorContainer => {
                self.error_palette
            }
            _ => self.neutral_palette,
        }
    }

    fn dynamic_tone(&self, role: Role) -> f64 {
        let decreasing_contrast = self.contrast_level < 0.0;

        if let Some(tone_delta_pair) = Self::tone_delta_pair(role) {
            return self.tone_from_delta_pair(role, tone_delta_pair, decreasing_contrast);
        }

        let mut answer = self.base_tone(role);
        let Some(background) = self.background(role) else {
            return answer;
        };

        let bg_tone = self.tone(background);
        let desired_ratio = Self::contrast_curve(role)
            .expect("background roles require a contrast curve")
            .get(self.contrast_level);

        if ratio_of_tones(bg_tone, answer) < desired_ratio {
            answer = foreground_tone(bg_tone, desired_ratio);
        }

        if decreasing_contrast {
            answer = foreground_tone(bg_tone, desired_ratio);
        }

        if Self::is_background(role) && (50.0..60.0).contains(&answer) {
            answer = if ratio_of_tones(49.0, bg_tone) >= desired_ratio {
                49.0
            } else {
                60.0
            };
        }

        if let Some(second_background) = Self::second_background(role) {
            return dual_background_tone(
                answer,
                bg_tone,
                self.tone(second_background),
                desired_ratio,
            );
        }

        answer
    }

    fn tone_from_delta_pair(
        &self,
        role: Role,
        pair: ToneDeltaPair,
        decreasing_contrast: bool,
    ) -> f64 {
        let bg_tone = self.tone(
            self.background(role)
                .expect("tone-delta roles require a background"),
        );
        let a_is_nearer = matches!(pair.polarity, TonePolarity::Nearer)
            || (matches!(pair.polarity, TonePolarity::Lighter) && !self.is_dark)
            || (matches!(pair.polarity, TonePolarity::Darker) && self.is_dark);
        let nearer = if a_is_nearer {
            pair.role_a
        } else {
            pair.role_b
        };
        let farther = if a_is_nearer {
            pair.role_b
        } else {
            pair.role_a
        };
        let am_nearer = role == nearer;
        let expansion_dir = if self.is_dark { 1.0 } else { -1.0 };
        let n_contrast = Self::contrast_curve(nearer)
            .expect("tone-delta roles require contrast curves")
            .get(self.contrast_level);
        let f_contrast = Self::contrast_curve(farther)
            .expect("tone-delta roles require contrast curves")
            .get(self.contrast_level);
        let n_initial_tone = self.base_tone(nearer);
        let f_initial_tone = self.base_tone(farther);
        let mut n_tone = if ratio_of_tones(bg_tone, n_initial_tone) >= n_contrast {
            n_initial_tone
        } else {
            foreground_tone(bg_tone, n_contrast)
        };
        let mut f_tone = if ratio_of_tones(bg_tone, f_initial_tone) >= f_contrast {
            f_initial_tone
        } else {
            foreground_tone(bg_tone, f_contrast)
        };

        if decreasing_contrast {
            n_tone = foreground_tone(bg_tone, n_contrast);
            f_tone = foreground_tone(bg_tone, f_contrast);
        }

        if (f_tone - n_tone) * expansion_dir < pair.delta {
            f_tone = (n_tone + pair.delta * expansion_dir).clamp(0.0, 100.0);
            if (f_tone - n_tone) * expansion_dir < pair.delta {
                n_tone = (f_tone - pair.delta * expansion_dir).clamp(0.0, 100.0);
            }
        }

        if (50.0..60.0).contains(&n_tone) {
            if expansion_dir > 0.0 {
                n_tone = 60.0;
                f_tone = f_tone.max(n_tone + pair.delta * expansion_dir);
            } else {
                n_tone = 49.0;
                f_tone = f_tone.min(n_tone + pair.delta * expansion_dir);
            }
        } else if (50.0..60.0).contains(&f_tone) {
            if pair.stay_together {
                if expansion_dir > 0.0 {
                    n_tone = 60.0;
                    f_tone = f_tone.max(n_tone + pair.delta * expansion_dir);
                } else {
                    n_tone = 49.0;
                    f_tone = f_tone.min(n_tone + pair.delta * expansion_dir);
                }
            } else if expansion_dir > 0.0 {
                f_tone = 60.0;
            } else {
                f_tone = 49.0;
            }
        }

        if am_nearer { n_tone } else { f_tone }
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn base_tone(&self, role: Role) -> f64 {
        match role {
            Role::PrimaryPaletteKeyColor
            | Role::SecondaryPaletteKeyColor
            | Role::TertiaryPaletteKeyColor
            | Role::NeutralPaletteKeyColor
            | Role::NeutralVariantPaletteKeyColor => self.palette(role).key_color().tone(),
            Role::Background | Role::Surface => {
                if self.is_dark {
                    6.0
                } else {
                    98.0
                }
            }
            Role::OnBackground | Role::OnSurface => {
                if self.is_dark {
                    90.0
                } else {
                    10.0
                }
            }
            Role::SurfaceDim => {
                if self.is_dark {
                    6.0
                } else {
                    ContrastCurve::new(87.0, 87.0, 80.0, 75.0).get(self.contrast_level)
                }
            }
            Role::SurfaceBright => {
                if self.is_dark {
                    ContrastCurve::new(24.0, 24.0, 29.0, 34.0).get(self.contrast_level)
                } else {
                    98.0
                }
            }
            Role::SurfaceContainerLowest => {
                if self.is_dark {
                    ContrastCurve::new(4.0, 4.0, 2.0, 0.0).get(self.contrast_level)
                } else {
                    100.0
                }
            }
            Role::SurfaceContainerLow => {
                if self.is_dark {
                    ContrastCurve::new(10.0, 10.0, 11.0, 12.0).get(self.contrast_level)
                } else {
                    ContrastCurve::new(96.0, 96.0, 96.0, 95.0).get(self.contrast_level)
                }
            }
            Role::SurfaceContainer => {
                if self.is_dark {
                    ContrastCurve::new(12.0, 12.0, 16.0, 20.0).get(self.contrast_level)
                } else {
                    ContrastCurve::new(94.0, 94.0, 92.0, 90.0).get(self.contrast_level)
                }
            }
            Role::SurfaceContainerHigh => {
                if self.is_dark {
                    ContrastCurve::new(17.0, 17.0, 21.0, 25.0).get(self.contrast_level)
                } else {
                    ContrastCurve::new(92.0, 92.0, 88.0, 85.0).get(self.contrast_level)
                }
            }
            Role::SurfaceContainerHighest => {
                if self.is_dark {
                    ContrastCurve::new(22.0, 22.0, 26.0, 30.0).get(self.contrast_level)
                } else {
                    ContrastCurve::new(90.0, 90.0, 84.0, 80.0).get(self.contrast_level)
                }
            }
            Role::SurfaceVariant => {
                if self.is_dark {
                    30.0
                } else {
                    90.0
                }
            }
            Role::OnSurfaceVariant => {
                if self.is_dark {
                    80.0
                } else {
                    30.0
                }
            }
            Role::InverseSurface => {
                if self.is_dark {
                    90.0
                } else {
                    20.0
                }
            }
            Role::InverseOnSurface => {
                if self.is_dark {
                    20.0
                } else {
                    95.0
                }
            }
            Role::Outline => {
                if self.is_dark {
                    60.0
                } else {
                    50.0
                }
            }
            Role::OutlineVariant => {
                if self.is_dark {
                    30.0
                } else {
                    80.0
                }
            }
            Role::Shadow | Role::Scrim => 0.0,
            Role::SurfaceTint => {
                if self.is_dark {
                    80.0
                } else {
                    40.0
                }
            }
            Role::Primary => {
                if self.is_monochrome() {
                    if self.is_dark { 100.0 } else { 0.0 }
                } else if self.is_dark {
                    80.0
                } else {
                    40.0
                }
            }
            Role::OnPrimary => {
                if self.is_monochrome() {
                    if self.is_dark { 10.0 } else { 90.0 }
                } else if self.is_dark {
                    20.0
                } else {
                    100.0
                }
            }
            Role::PrimaryContainer => {
                if self.is_fidelity() {
                    self.source_color_hct.tone()
                } else if self.is_monochrome() {
                    if self.is_dark { 85.0 } else { 25.0 }
                } else if self.is_dark {
                    30.0
                } else {
                    90.0
                }
            }
            Role::OnPrimaryContainer => {
                if self.is_fidelity() {
                    foreground_tone(self.base_tone(Role::PrimaryContainer), 4.5)
                } else if self.is_monochrome() {
                    if self.is_dark { 0.0 } else { 100.0 }
                } else if self.is_dark {
                    90.0
                } else {
                    30.0
                }
            }
            Role::InversePrimary => {
                if self.is_dark {
                    40.0
                } else {
                    80.0
                }
            }
            Role::Secondary => {
                if self.is_dark {
                    80.0
                } else {
                    40.0
                }
            }
            Role::OnSecondary => {
                if self.is_monochrome() {
                    if self.is_dark { 10.0 } else { 100.0 }
                } else if self.is_dark {
                    20.0
                } else {
                    100.0
                }
            }
            Role::SecondaryContainer => {
                let initial_tone = if self.is_dark { 30.0 } else { 90.0 };
                if self.is_monochrome() {
                    if self.is_dark { 30.0 } else { 85.0 }
                } else if self.is_fidelity() {
                    find_desired_chroma_by_tone(
                        self.secondary_palette.hue(),
                        self.secondary_palette.chroma(),
                        initial_tone,
                        !self.is_dark,
                    )
                } else {
                    initial_tone
                }
            }
            Role::OnSecondaryContainer => {
                if self.is_monochrome() {
                    if self.is_dark { 90.0 } else { 10.0 }
                } else if self.is_fidelity() {
                    foreground_tone(self.base_tone(Role::SecondaryContainer), 4.5)
                } else if self.is_dark {
                    90.0
                } else {
                    30.0
                }
            }
            Role::Tertiary => {
                if self.is_monochrome() {
                    if self.is_dark { 90.0 } else { 25.0 }
                } else if self.is_dark {
                    80.0
                } else {
                    40.0
                }
            }
            Role::OnTertiary => {
                if self.is_monochrome() {
                    if self.is_dark { 10.0 } else { 90.0 }
                } else if self.is_dark {
                    20.0
                } else {
                    100.0
                }
            }
            Role::TertiaryContainer => {
                if self.is_monochrome() {
                    if self.is_dark { 60.0 } else { 49.0 }
                } else if self.is_fidelity() {
                    let proposed_hct =
                        Hct::from_argb(self.tertiary_palette.get(self.source_color_hct.tone()));
                    fix_if_disliked(proposed_hct).tone()
                } else if self.is_dark {
                    30.0
                } else {
                    90.0
                }
            }
            Role::OnTertiaryContainer => {
                if self.is_monochrome() {
                    if self.is_dark { 0.0 } else { 100.0 }
                } else if self.is_fidelity() {
                    foreground_tone(self.base_tone(Role::TertiaryContainer), 4.5)
                } else if self.is_dark {
                    90.0
                } else {
                    30.0
                }
            }
            Role::Error => {
                if self.is_dark {
                    80.0
                } else {
                    40.0
                }
            }
            Role::OnError => {
                if self.is_dark {
                    20.0
                } else {
                    100.0
                }
            }
            Role::ErrorContainer => {
                if self.is_dark {
                    30.0
                } else {
                    90.0
                }
            }
            Role::OnErrorContainer => {
                if self.is_monochrome() {
                    if self.is_dark { 90.0 } else { 10.0 }
                } else if self.is_dark {
                    90.0
                } else {
                    30.0
                }
            }
            Role::PrimaryFixed => {
                if self.is_monochrome() {
                    40.0
                } else {
                    90.0
                }
            }
            Role::PrimaryFixedDim => {
                if self.is_monochrome() {
                    30.0
                } else {
                    80.0
                }
            }
            Role::OnPrimaryFixed => {
                if self.is_monochrome() {
                    100.0
                } else {
                    10.0
                }
            }
            Role::OnPrimaryFixedVariant => {
                if self.is_monochrome() {
                    90.0
                } else {
                    30.0
                }
            }
            Role::SecondaryFixed => {
                if self.is_monochrome() {
                    80.0
                } else {
                    90.0
                }
            }
            Role::SecondaryFixedDim => {
                if self.is_monochrome() {
                    70.0
                } else {
                    80.0
                }
            }
            Role::OnSecondaryFixed => 10.0,
            Role::OnSecondaryFixedVariant => {
                if self.is_monochrome() {
                    25.0
                } else {
                    30.0
                }
            }
            Role::TertiaryFixed => {
                if self.is_monochrome() {
                    40.0
                } else {
                    90.0
                }
            }
            Role::TertiaryFixedDim => {
                if self.is_monochrome() {
                    30.0
                } else {
                    80.0
                }
            }
            Role::OnTertiaryFixed => {
                if self.is_monochrome() {
                    100.0
                } else {
                    10.0
                }
            }
            Role::OnTertiaryFixedVariant => {
                if self.is_monochrome() {
                    90.0
                } else {
                    30.0
                }
            }
        }
    }

    const fn is_fidelity(&self) -> bool {
        matches!(self.variant, Variant::Fidelity | Variant::Content)
    }

    const fn is_monochrome(&self) -> bool {
        matches!(self.variant, Variant::Monochrome)
    }

    const fn highest_surface(&self) -> Role {
        if self.is_dark {
            Role::SurfaceBright
        } else {
            Role::SurfaceDim
        }
    }

    #[allow(clippy::match_same_arms)]
    const fn background(&self, role: Role) -> Option<Role> {
        match role {
            Role::OnBackground => Some(Role::Background),
            Role::OnSurface
            | Role::OnSurfaceVariant
            | Role::Outline
            | Role::OutlineVariant
            | Role::Primary
            | Role::PrimaryContainer
            | Role::Secondary
            | Role::SecondaryContainer
            | Role::Tertiary
            | Role::TertiaryContainer
            | Role::Error
            | Role::ErrorContainer
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim => Some(self.highest_surface()),
            Role::InverseOnSurface | Role::InversePrimary => Some(Role::InverseSurface),
            Role::OnPrimary => Some(Role::Primary),
            Role::OnPrimaryContainer => Some(Role::PrimaryContainer),
            Role::OnSecondary => Some(Role::Secondary),
            Role::OnSecondaryContainer => Some(Role::SecondaryContainer),
            Role::OnTertiary => Some(Role::Tertiary),
            Role::OnTertiaryContainer => Some(Role::TertiaryContainer),
            Role::OnError => Some(Role::Error),
            Role::OnErrorContainer => Some(Role::ErrorContainer),
            Role::OnPrimaryFixed | Role::OnPrimaryFixedVariant => Some(Role::PrimaryFixedDim),
            Role::OnSecondaryFixed | Role::OnSecondaryFixedVariant => Some(Role::SecondaryFixedDim),
            Role::OnTertiaryFixed | Role::OnTertiaryFixedVariant => Some(Role::TertiaryFixedDim),
            _ => None,
        }
    }

    const fn second_background(role: Role) -> Option<Role> {
        match role {
            Role::OnPrimaryFixed | Role::OnPrimaryFixedVariant => Some(Role::PrimaryFixed),
            Role::OnSecondaryFixed | Role::OnSecondaryFixedVariant => Some(Role::SecondaryFixed),
            Role::OnTertiaryFixed | Role::OnTertiaryFixedVariant => Some(Role::TertiaryFixed),
            _ => None,
        }
    }

    #[allow(clippy::match_same_arms)]
    const fn contrast_curve(role: Role) -> Option<ContrastCurve> {
        match role {
            Role::OnBackground => Some(ContrastCurve::new(3.0, 3.0, 4.5, 7.0)),
            Role::OnSurface
            | Role::InverseOnSurface
            | Role::OnPrimary
            | Role::OnSecondary
            | Role::OnTertiary
            | Role::OnError
            | Role::OnPrimaryFixed
            | Role::OnSecondaryFixed
            | Role::OnTertiaryFixed => Some(ContrastCurve::new(4.5, 7.0, 11.0, 21.0)),
            Role::OnSurfaceVariant
            | Role::OnPrimaryContainer
            | Role::OnSecondaryContainer
            | Role::OnTertiaryContainer
            | Role::OnErrorContainer
            | Role::OnPrimaryFixedVariant
            | Role::OnSecondaryFixedVariant
            | Role::OnTertiaryFixedVariant => Some(ContrastCurve::new(3.0, 4.5, 7.0, 11.0)),
            Role::Outline => Some(ContrastCurve::new(1.5, 3.0, 4.5, 7.0)),
            Role::OutlineVariant => Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            Role::Primary
            | Role::InversePrimary
            | Role::Secondary
            | Role::Tertiary
            | Role::Error => Some(ContrastCurve::new(3.0, 4.5, 7.0, 7.0)),
            Role::PrimaryContainer
            | Role::SecondaryContainer
            | Role::TertiaryContainer
            | Role::ErrorContainer
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim => Some(ContrastCurve::new(1.0, 1.0, 3.0, 4.5)),
            _ => None,
        }
    }

    const fn tone_delta_pair(role: Role) -> Option<ToneDeltaPair> {
        match role {
            Role::Primary | Role::PrimaryContainer => Some(ToneDeltaPair::new(
                Role::PrimaryContainer,
                Role::Primary,
                10.0,
                TonePolarity::Nearer,
                false,
            )),
            Role::Secondary | Role::SecondaryContainer => Some(ToneDeltaPair::new(
                Role::SecondaryContainer,
                Role::Secondary,
                10.0,
                TonePolarity::Nearer,
                false,
            )),
            Role::Tertiary | Role::TertiaryContainer => Some(ToneDeltaPair::new(
                Role::TertiaryContainer,
                Role::Tertiary,
                10.0,
                TonePolarity::Nearer,
                false,
            )),
            Role::Error | Role::ErrorContainer => Some(ToneDeltaPair::new(
                Role::ErrorContainer,
                Role::Error,
                10.0,
                TonePolarity::Nearer,
                false,
            )),
            Role::PrimaryFixed | Role::PrimaryFixedDim => Some(ToneDeltaPair::new(
                Role::PrimaryFixed,
                Role::PrimaryFixedDim,
                10.0,
                TonePolarity::Lighter,
                true,
            )),
            Role::SecondaryFixed | Role::SecondaryFixedDim => Some(ToneDeltaPair::new(
                Role::SecondaryFixed,
                Role::SecondaryFixedDim,
                10.0,
                TonePolarity::Lighter,
                true,
            )),
            Role::TertiaryFixed | Role::TertiaryFixedDim => Some(ToneDeltaPair::new(
                Role::TertiaryFixed,
                Role::TertiaryFixedDim,
                10.0,
                TonePolarity::Lighter,
                true,
            )),
            _ => None,
        }
    }

    const fn is_background(role: Role) -> bool {
        matches!(
            role,
            Role::Background
                | Role::Surface
                | Role::SurfaceDim
                | Role::SurfaceBright
                | Role::SurfaceContainerLowest
                | Role::SurfaceContainerLow
                | Role::SurfaceContainer
                | Role::SurfaceContainerHigh
                | Role::SurfaceContainerHighest
                | Role::SurfaceVariant
                | Role::SurfaceTint
                | Role::Primary
                | Role::PrimaryContainer
                | Role::Secondary
                | Role::SecondaryContainer
                | Role::Tertiary
                | Role::TertiaryContainer
                | Role::Error
                | Role::ErrorContainer
                | Role::PrimaryFixed
                | Role::PrimaryFixedDim
                | Role::SecondaryFixed
                | Role::SecondaryFixedDim
                | Role::TertiaryFixed
                | Role::TertiaryFixedDim
        )
    }
}

#[must_use]
fn find_desired_chroma_by_tone(hue: f64, chroma: f64, tone: f64, by_decreasing_tone: bool) -> f64 {
    let mut answer = tone;
    let mut closest_to_chroma = Hct::new(hue, chroma, tone);

    if closest_to_chroma.chroma() < chroma {
        let mut chroma_peak = closest_to_chroma.chroma();
        while closest_to_chroma.chroma() < chroma {
            answer += if by_decreasing_tone { -1.0 } else { 1.0 };
            let potential_solution = Hct::new(hue, chroma, answer);
            if chroma_peak > potential_solution.chroma() {
                break;
            }
            if (potential_solution.chroma() - chroma).abs() < 0.4 {
                break;
            }

            let potential_delta = (potential_solution.chroma() - chroma).abs();
            let current_delta = (closest_to_chroma.chroma() - chroma).abs();
            if potential_delta < current_delta {
                closest_to_chroma = potential_solution;
            }
            chroma_peak = chroma_peak.max(potential_solution.chroma());
        }
    }

    answer
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
    fn role_inventory_matches_material_dynamic_colors() {
        assert_eq!(DynamicColorRole::ALL.len(), 54);
        assert_eq!(DynamicColorRole::Primary.name(), "primary");
        assert_eq!(
            DynamicColorRole::NeutralVariantPaletteKeyColor.name(),
            "neutral_variant_palette_key_color"
        );
    }

    #[test]
    fn monochrome_dark_theme_matches_cpp_spec_tones() {
        let scheme = DynamicScheme::monochrome(Hct::from_argb(Argb::new(0xff00_00ff)), true, 0.0);

        for (role, expected) in [
            (Role::Primary, 100.0),
            (Role::OnPrimary, 10.0),
            (Role::PrimaryContainer, 85.0),
            (Role::OnPrimaryContainer, 0.0),
            (Role::Secondary, 80.0),
            (Role::OnSecondary, 10.0),
            (Role::SecondaryContainer, 30.0),
            (Role::OnSecondaryContainer, 90.0),
            (Role::Tertiary, 90.0),
            (Role::OnTertiary, 10.0),
            (Role::TertiaryContainer, 60.0),
            (Role::OnTertiaryContainer, 0.0),
        ] {
            assert_near(scheme.hct(role).tone(), expected, 1.0);
        }
    }

    #[test]
    fn monochrome_light_theme_matches_cpp_spec_tones() {
        let scheme = DynamicScheme::monochrome(Hct::from_argb(Argb::new(0xff00_00ff)), false, 0.0);

        for (role, expected) in [
            (Role::Primary, 0.0),
            (Role::OnPrimary, 90.0),
            (Role::PrimaryContainer, 25.0),
            (Role::OnPrimaryContainer, 100.0),
            (Role::Secondary, 40.0),
            (Role::OnSecondary, 100.0),
            (Role::SecondaryContainer, 85.0),
            (Role::OnSecondaryContainer, 10.0),
            (Role::Tertiary, 25.0),
            (Role::OnTertiary, 90.0),
            (Role::TertiaryContainer, 49.0),
            (Role::OnTertiaryContainer, 100.0),
        ] {
            assert_near(scheme.hct(role).tone(), expected, 1.0);
        }
    }

    #[test]
    fn every_role_generates_opaque_color() {
        let scheme = DynamicScheme::tonal_spot(Hct::from_argb(Argb::new(0xff42_85f4)), false, 0.0);

        for role in DynamicColorRole::ALL {
            assert_eq!(scheme.color(role).alpha(), 0xff, "role {}", role.name());
        }
    }

    #[test]
    fn fidelity_and_content_use_temperature_palettes() {
        let source = Hct::from_argb(Argb::new(0xff00_00ff));
        let fidelity = DynamicScheme::fidelity(source, false, 0.0);
        let content = DynamicScheme::content(source, false, 0.0);

        assert_eq!(
            hex_from_argb(fidelity.tertiary_palette().key_color().to_argb()),
            "ff9d0002"
        );
        assert_eq!(
            hex_from_argb(content.tertiary_palette().key_color().to_argb()),
            "ff81009f"
        );
    }
}
