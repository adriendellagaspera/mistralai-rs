pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SearchRequestRetriever {
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

    #[serde(rename = "rrf")]
    #[non_exhaustive]
    Rrf {
        #[serde(skip_serializing_if = "Option::is_none")]
        filter: Option<RrfRetrieverFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rank_constant: Option<i64>,
        #[serde(default)]
        retrievers: Vec<RrfRetrieverRetrieversItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        top_k: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        weights: Option<Vec<f64>>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SearchRequestRetriever {
    pub fn keyword(data: KeywordRetriever) -> Self {
        Self::Keyword { data }
    }

    pub fn nearest_neighbour(data: NearestNeighbourRetriever) -> Self {
        Self::NearestNeighbour { data }
    }

    pub fn rrf(retrievers: Vec<RrfRetrieverRetrieversItem>) -> Self {
        Self::Rrf {
            filter: None,
            rank_constant: None,
            retrievers,
            top_k: None,
            weights: None,
        }
    }

    pub fn rrf_with_filter(
        filter: RrfRetrieverFilter,
        rank_constant: Option<i64>,
        retrievers: Vec<RrfRetrieverRetrieversItem>,
        top_k: Option<i64>,
        weights: Option<Vec<f64>>,
    ) -> Self {
        Self::Rrf {
            filter: Some(filter),
            rank_constant,
            retrievers,
            top_k,
            weights,
        }
    }

    pub fn rrf_with_rank_constant(
        filter: Option<RrfRetrieverFilter>,
        rank_constant: i64,
        retrievers: Vec<RrfRetrieverRetrieversItem>,
        top_k: Option<i64>,
        weights: Option<Vec<f64>>,
    ) -> Self {
        Self::Rrf {
            filter,
            rank_constant: Some(rank_constant),
            retrievers,
            top_k,
            weights,
        }
    }

    pub fn rrf_with_top_k(
        filter: Option<RrfRetrieverFilter>,
        rank_constant: Option<i64>,
        retrievers: Vec<RrfRetrieverRetrieversItem>,
        top_k: i64,
        weights: Option<Vec<f64>>,
    ) -> Self {
        Self::Rrf {
            filter,
            rank_constant,
            retrievers,
            top_k: Some(top_k),
            weights,
        }
    }

    pub fn rrf_with_weights(
        filter: Option<RrfRetrieverFilter>,
        rank_constant: Option<i64>,
        retrievers: Vec<RrfRetrieverRetrieversItem>,
        top_k: Option<i64>,
        weights: Vec<f64>,
    ) -> Self {
        Self::Rrf {
            filter,
            rank_constant,
            retrievers,
            top_k,
            weights: Some(weights),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
