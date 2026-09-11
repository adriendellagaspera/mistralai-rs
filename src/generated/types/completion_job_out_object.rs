///The object type of the fine-tuning job.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionJobOutObject {
    #[default]
    #[serde(rename = "job")]
    Job,
}
