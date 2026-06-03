#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Variant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad,
}

impl Variant {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Monochrome => "monochrome",
            Self::Neutral => "neutral",
            Self::TonalSpot => "tonal_spot",
            Self::Vibrant => "vibrant",
            Self::Expressive => "expressive",
            Self::Fidelity => "fidelity",
            Self::Content => "content",
            Self::Rainbow => "rainbow",
            Self::FruitSalad => "fruit_salad",
        }
    }
}
