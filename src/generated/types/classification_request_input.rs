///Text to classify.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClassificationRequestInput {
    String(String),
    ClassificationRequestInputStringArray(ClassificationRequestInputStringArray),
}
