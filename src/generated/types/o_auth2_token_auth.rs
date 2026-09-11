#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuth2TokenAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OAuth2TokenAuthType>,
    pub value: String,
}
