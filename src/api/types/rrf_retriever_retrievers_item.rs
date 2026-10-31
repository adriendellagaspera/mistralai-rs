pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RrfRetrieverRetrieversItem {
    #[serde(rename = "keyword")]
    #[non_exhaustive]
    Keyword {
        #[serde(flatten)]
        data: KeywordRetriever,
    },

    #[serde(rename = "nearest_neighbour")]
    #[non_exhaustive]
    NearestNeighbour {
        #[serde(flatten)]
        data: NearestNeighbourRetriever,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl RrfRetrieverRetrieversItem {
    pub fn keyword(data: KeywordRetriever) -> Self {
        Self::Keyword { data }
    }

    pub fn nearest_neighbour(data: NearestNeighbourRetriever) -> Self {
        Self::NearestNeighbour { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
