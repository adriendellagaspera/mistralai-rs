pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmbeddingResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

impl EmbeddingResponseData {
    pub fn builder() -> EmbeddingResponseDataBuilder {
        <EmbeddingResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbeddingResponseDataBuilder {
    object: Option<String>,
    embedding: Option<Vec<f64>>,
    index: Option<i64>,
}

impl EmbeddingResponseDataBuilder {
    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn embedding(mut self, value: Vec<f64>) -> Self {
        self.embedding = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmbeddingResponseData`].
    pub fn build(self) -> Result<EmbeddingResponseData, BuildError> {
        Ok(EmbeddingResponseData {
            object: self.object,
            embedding: self.embedding,
            index: self.index,
        })
    }
}
