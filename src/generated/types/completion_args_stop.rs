#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CompletionArgsStop {
    String(String),
    CompletionArgsStopStringArray(CompletionArgsStopStringArray),
}
