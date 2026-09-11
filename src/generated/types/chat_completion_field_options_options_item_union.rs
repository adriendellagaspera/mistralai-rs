#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatCompletionFieldOptionsOptionsItemUnion {
    String(String),
    Boolean(bool),
}
