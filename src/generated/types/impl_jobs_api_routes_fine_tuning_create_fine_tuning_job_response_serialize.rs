impl Serialize for JobsApiRoutesFineTuningCreateFineTuningJobResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::JobsApiRoutesFineTuningCreateFineTuningJobResponseInlineVariant(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::LegacyJobMetadataOut(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}
