#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TimestampGranularity {
    #[default]
    #[serde(rename = "segment")]
    Segment,
    #[serde(rename = "word")]
    Word,
}
