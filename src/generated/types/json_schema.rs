#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonSchema {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    pub name: String,
    pub schema: JsonSchemaSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}
