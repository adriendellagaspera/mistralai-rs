#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchRequest {
    pub body: BatchRequestBody,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub custom_id: Option<Option<String>>,
}
