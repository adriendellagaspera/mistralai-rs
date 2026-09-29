pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CustomTaskInProgressAttributesResponsePayload {
    #[serde(rename = "json")]
    #[non_exhaustive]
    Json {
        #[serde(flatten)]
        data: JsonPayloadResponse,
    },

    #[serde(rename = "json_patch")]
    #[non_exhaustive]
    JsonPatch {
        #[serde(skip_serializing_if = "Option::is_none")]
        encoding_options: Option<Vec<EncodedPayloadOptions>>,
        value: JsonPatchPayloadValueResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CustomTaskInProgressAttributesResponsePayload {
    pub fn json(data: JsonPayloadResponse) -> Self {
        Self::Json { data }
    }

    pub fn json_patch(value: JsonPatchPayloadValueResponse) -> Self {
        Self::JsonPatch {
            encoding_options: None,
            value,
        }
    }

    pub fn json_patch_with_encoding_options(
        encoding_options: Vec<EncodedPayloadOptions>,
        value: JsonPatchPayloadValueResponse,
    ) -> Self {
        Self::JsonPatch {
            encoding_options: Some(encoding_options),
            value,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
