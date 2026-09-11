#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateDefinition {
    ///Description of the update
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///Input JSON schema of the update's model
    pub input_schema: UpdateDefinitionInputSchema,
    ///Name of the update
    pub name: String,
    ///Output JSON schema of the update's model
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_schema: Option<Option<UpdateDefinitionOutputSchema>>,
}
