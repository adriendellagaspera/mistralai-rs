#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkEncodedInput {
    ///The encoded payload
    pub b64payload: String,
    ///Whether the payload is empty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    ///The encoding of the payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_options: Option<Vec<EncodedPayloadOptions>>,
}
