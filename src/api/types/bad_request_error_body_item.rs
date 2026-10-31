pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BadRequestErrorBodyItem {
    #[serde(rename = "embedding_mismatch")]
    #[non_exhaustive]
    EmbeddingMismatch {
        #[serde(default)]
        expected: i64,
        #[serde(default)]
        received: i64,
    },

    #[serde(rename = "incorrect_field_type")]
    #[non_exhaustive]
    IncorrectFieldType {
        #[serde(default)]
        expected_type: String,
        #[serde(default)]
        name: String,
        #[serde(default)]
        received_type: String,
    },

    #[serde(rename = "max_candidates_too_small")]
    #[non_exhaustive]
    MaxCandidatesTooSmall {
        #[serde(default)]
        max_candidates: i64,
        #[serde(default)]
        top_k: i64,
    },

    #[serde(rename = "missing_embedding")]
    #[non_exhaustive]
    MissingEmbedding {},

    #[serde(rename = "missing_field")]
    #[non_exhaustive]
    MissingField {
        #[serde(default)]
        name: String,
    },

    #[serde(rename = "top_k_too_large")]
    #[non_exhaustive]
    TopKTooLarge {
        #[serde(default)]
        limit: i64,
        #[serde(default)]
        top_k: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl BadRequestErrorBodyItem {
    pub fn embedding_mismatch(expected: i64, received: i64) -> Self {
        Self::EmbeddingMismatch { expected, received }
    }

    pub fn incorrect_field_type(
        expected_type: String,
        name: String,
        received_type: String,
    ) -> Self {
        Self::IncorrectFieldType {
            expected_type,
            name,
            received_type,
        }
    }

    pub fn max_candidates_too_small(max_candidates: i64, top_k: i64) -> Self {
        Self::MaxCandidatesTooSmall {
            max_candidates,
            top_k,
        }
    }

    pub fn missing_embedding() -> Self {
        Self::MissingEmbedding {}
    }

    pub fn missing_field(name: String) -> Self {
        Self::MissingField { name }
    }

    pub fn top_k_too_large(limit: i64, top_k: i64) -> Self {
        Self::TopKTooLarge { limit, top_k }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
