pub use crate::prelude::*;

/// Retrieve document chunks whose embeddings are closest to the query's.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NearestNeighbourRetriever {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NearestNeighbourRetrieverFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_candidates: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_embedding: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
}

impl NearestNeighbourRetriever {
    pub fn builder() -> NearestNeighbourRetrieverBuilder {
        <NearestNeighbourRetrieverBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NearestNeighbourRetrieverBuilder {
    field: Option<String>,
    filter: Option<NearestNeighbourRetrieverFilter>,
    max_candidates: Option<i64>,
    query: Option<String>,
    query_embedding: Option<Vec<f64>>,
    top_k: Option<i64>,
}

impl NearestNeighbourRetrieverBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn filter(mut self, value: NearestNeighbourRetrieverFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn max_candidates(mut self, value: i64) -> Self {
        self.max_candidates = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn query_embedding(mut self, value: Vec<f64>) -> Self {
        self.query_embedding = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NearestNeighbourRetriever`].
    pub fn build(self) -> Result<NearestNeighbourRetriever, BuildError> {
        Ok(NearestNeighbourRetriever {
            field: self.field,
            filter: self.filter,
            max_candidates: self.max_candidates,
            query: self.query,
            query_embedding: self.query_embedding,
            top_k: self.top_k,
        })
    }
}
