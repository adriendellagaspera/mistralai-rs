impl ReferenceChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reference => "reference",
        }
    }
}
