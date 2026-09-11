impl JudgeOutputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Regression => "REGRESSION",
            Self::Classification => "CLASSIFICATION",
        }
    }
}
