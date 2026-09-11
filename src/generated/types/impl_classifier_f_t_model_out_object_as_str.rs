impl ClassifierFTModelOutObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Model => "model",
        }
    }
}
