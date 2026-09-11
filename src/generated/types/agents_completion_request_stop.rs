///Stop generation if this token is detected. Or if one of these tokens is detected when providing an array
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AgentsCompletionRequestStop {
    String(String),
    AgentsCompletionRequestStopStringArray(AgentsCompletionRequestStopStringArray),
}
