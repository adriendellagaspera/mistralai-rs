pub use crate::prelude::*;

/// A payload containing arbitrary JSON data.
///
/// Used for complete state snapshots or final results.
/// When encrypted, the value field contains base64-encoded encrypted data
/// and encoding_options indicates the type of encryption applied.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonPayloadResponse {
    /// Encoding options applied to the payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_options: Option<Vec<EncodedPayloadOptions>>,
    /// Discriminator indicating this is a raw JSON payload.
    pub r#type: JsonPayloadResponseType,
    pub value: serde_json::Value,
}

impl JsonPayloadResponse {
    pub fn builder() -> JsonPayloadResponseBuilder {
        <JsonPayloadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JsonPayloadResponseBuilder {
    encoding_options: Option<Vec<EncodedPayloadOptions>>,
    r#type: Option<JsonPayloadResponseType>,
    value: Option<serde_json::Value>,
}

impl JsonPayloadResponseBuilder {
    pub fn encoding_options(mut self, value: Vec<EncodedPayloadOptions>) -> Self {
        self.encoding_options = Some(value);
        self
    }

    pub fn r#type(mut self, value: JsonPayloadResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JsonPayloadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](JsonPayloadResponseBuilder::r#type)
    /// - [`value`](JsonPayloadResponseBuilder::value)
    pub fn build(self) -> Result<JsonPayloadResponse, BuildError> {
        Ok(JsonPayloadResponse {
            encoding_options: self.encoding_options,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
