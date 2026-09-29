pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmbeddingResponse {
    #[serde(flatten)]
    pub response_base_fields: ResponseBase,
    #[serde(default)]
    pub data: Vec<EmbeddingResponseData>,
}

impl EmbeddingResponse {
    pub fn builder() -> EmbeddingResponseBuilder {
        <EmbeddingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbeddingResponseBuilder {
    response_base_fields: Option<ResponseBase>,
    data: Option<Vec<EmbeddingResponseData>>,
}

impl EmbeddingResponseBuilder {
    pub fn response_base_fields(mut self, value: ResponseBase) -> Self {
        self.response_base_fields = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<EmbeddingResponseData>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmbeddingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_base_fields`](EmbeddingResponseBuilder::response_base_fields)
    /// - [`data`](EmbeddingResponseBuilder::data)
    pub fn build(self) -> Result<EmbeddingResponse, BuildError> {
        Ok(EmbeddingResponse {
            response_base_fields: self
                .response_base_fields
                .ok_or_else(|| BuildError::missing_field("response_base_fields"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
