#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ImageGenerationTool {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tool_configuration: Option<Option<ToolConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ImageGenerationToolType>,
}
