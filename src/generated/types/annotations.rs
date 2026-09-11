#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Annotations {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub audience: Option<Option<Vec<AnnotationsAudienceItem>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub priority: Option<Option<f64>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
