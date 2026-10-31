pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchHitResponse {
    #[serde(default)]
    pub chunk: SearchChunkResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(default)]
    pub index_name: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
}

impl SearchHitResponse {
    pub fn builder() -> SearchHitResponseBuilder {
        <SearchHitResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchHitResponseBuilder {
    chunk: Option<SearchChunkResponse>,
    distance: Option<f64>,
    index_name: Option<String>,
    score: Option<f64>,
}

impl SearchHitResponseBuilder {
    pub fn chunk(mut self, value: SearchChunkResponse) -> Self {
        self.chunk = Some(value);
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    pub fn index_name(mut self, value: impl Into<String>) -> Self {
        self.index_name = Some(value.into());
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchHitResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk`](SearchHitResponseBuilder::chunk)
    /// - [`index_name`](SearchHitResponseBuilder::index_name)
    /// - [`score`](SearchHitResponseBuilder::score)
    pub fn build(self) -> Result<SearchHitResponse, BuildError> {
        Ok(SearchHitResponse {
            chunk: self
                .chunk
                .ok_or_else(|| BuildError::missing_field("chunk"))?,
            distance: self.distance,
            index_name: self
                .index_name
                .ok_or_else(|| BuildError::missing_field("index_name"))?,
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
        })
    }
}
