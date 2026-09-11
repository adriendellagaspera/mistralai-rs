#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignalDefinition {
    ///Description of the signal
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///Input JSON schema of the signal's model
    pub input_schema: SignalDefinitionInputSchema,
    ///Name of the signal
    pub name: String,
}
