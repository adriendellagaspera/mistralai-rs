///ToolChoice is either a ToolChoiceEnum or a ToolChoice
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolChoice {
    pub function: FunctionName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolTypes>,
}
