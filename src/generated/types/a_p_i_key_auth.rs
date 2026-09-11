#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIKeyAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<APIKeyAuthType>,
    pub value: String,
}
