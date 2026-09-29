pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmbeddingResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

impl EmbeddingResponseData {
    pub fn builder() -> EmbeddingResponseDataBuilder {
        <EmbeddingResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbeddingResponseDataBuilder {
    embedding: Option<Vec<f64>>,
    index: Option<i64>,
    object: Option<String>,
}

impl EmbeddingResponseDataBuilder {
    pub fn embedding(mut self, value: Vec<f64>) -> Self {
        self.embedding = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmbeddingResponseData`].
    pub fn build(self) -> Result<EmbeddingResponseData, BuildError> {
        Ok(EmbeddingResponseData {
            embedding: self.embedding,
            index: self.index,
            object: self.object,
        })
    }
}
