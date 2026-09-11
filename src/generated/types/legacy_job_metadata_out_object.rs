#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum LegacyJobMetadataOutObject {
    #[default]
    #[serde(rename = "job.metadata")]
    JobMetadata,
}
