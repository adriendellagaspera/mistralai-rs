impl FTClassifierLossFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleClass => "single_class",
            Self::MultiClass => "multi_class",
        }
    }
}
