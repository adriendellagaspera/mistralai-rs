#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionDetailedJobOutObject {
    #[default]
    #[serde(rename = "job")]
    Job,
}
