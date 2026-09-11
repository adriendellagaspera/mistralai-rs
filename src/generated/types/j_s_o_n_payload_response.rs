/**A payload containing arbitrary JSON data.

Used for complete state snapshots or final results.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JSONPayloadResponse {
    ///Discriminator indicating this is a raw JSON payload.
    #[serde(default)]
    pub r#type: JSONPayloadResponseType,
    ///The JSON-serializable payload value.
    pub value: serde_json::Value,
}
