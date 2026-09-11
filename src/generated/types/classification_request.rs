#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassificationRequest {
    ///Text to classify.
    pub input: ClassificationRequestInput,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<ClassificationRequestMetadata>>,
    ///ID of the model to use.
    pub model: String,
}
