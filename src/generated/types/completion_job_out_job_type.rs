///The type of job (`FT` for fine-tuning).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionJobOutJobType {
    #[default]
    #[serde(rename = "completion")]
    Completion,
}
