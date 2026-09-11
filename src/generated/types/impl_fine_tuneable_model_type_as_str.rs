impl FineTuneableModelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Classifier => "classifier",
        }
    }
}
