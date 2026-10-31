pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum IngestDocumentsResponseResultsItem {
    #[serde(rename = "accepted")]
    #[non_exhaustive]
    Accepted {
        #[serde(default)]
        chunk_count: i64,
        #[serde(default)]
        document_id: String,
        #[serde(default)]
        position: i64,
    },

    #[serde(rename = "rejected")]
    #[non_exhaustive]
    Rejected {
        #[serde(default)]
        errors: Vec<DocumentFieldErrorResponse>,
        #[serde(default)]
        position: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl IngestDocumentsResponseResultsItem {
    pub fn accepted(chunk_count: i64, document_id: String, position: i64) -> Self {
        Self::Accepted {
            chunk_count,
            document_id,
            position,
        }
    }

    pub fn rejected(errors: Vec<DocumentFieldErrorResponse>, position: i64) -> Self {
        Self::Rejected { errors, position }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
