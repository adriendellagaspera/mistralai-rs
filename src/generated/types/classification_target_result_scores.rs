#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ClassificationTargetResultScores {
    /// Additional properties matching the spec's
    /// `additionalProperties` value schema.
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, f64>,
}
