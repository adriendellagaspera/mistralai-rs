#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConnectionCredentialsHeaders {
    /// Additional properties matching the spec's
    /// `additionalProperties` value schema.
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, String>,
}
