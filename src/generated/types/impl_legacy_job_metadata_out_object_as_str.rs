impl LegacyJobMetadataOutObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::JobMetadata => "job.metadata",
        }
    }
}
