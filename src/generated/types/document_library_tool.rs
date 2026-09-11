#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentLibraryTool {
    ///Ids of the library in which to search.
    ///Constraint: minItems=1
    pub library_ids: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tool_configuration: Option<Option<ToolConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DocumentLibraryToolType>,
}
