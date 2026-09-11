#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BaseFieldDefinition {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub group: Option<Option<String>>,
    pub label: String,
    pub name: String,
    pub supported_operators: Vec<BaseFieldDefinitionSupportedOperatorsItem>,
    pub r#type: BaseFieldDefinitionType,
}
