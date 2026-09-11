#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryDefinition {
    ///Description of the query
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///Input JSON schema of the query's model
    pub input_schema: QueryDefinitionInputSchema,
    ///Name of the query
    pub name: String,
    ///Output JSON schema of the query's model
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_schema: Option<Option<QueryDefinitionOutputSchema>>,
}
