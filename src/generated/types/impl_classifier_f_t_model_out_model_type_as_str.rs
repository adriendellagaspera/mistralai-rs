impl ClassifierFTModelOutModelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Classifier => "classifier",
        }
    }
}
