///Discriminator indicating this is a raw JSON payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPayloadResponseType {
    #[default]
    #[serde(rename = "json")]
    Json,
}
