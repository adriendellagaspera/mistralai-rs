#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AgentsCompletionRequestToolChoice {
    ToolChoice(ToolChoice),
    ToolChoiceEnum(ToolChoiceEnum),
}
