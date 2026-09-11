impl ImageDetail {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Auto => "auto",
            Self::High => "high",
        }
    }
}
