pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingRequest {
    /// ID of the model to use.
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Text to embed.
    pub input: EmbeddingRequestInput,
    /// The dimension of the output embeddings when feature available. If not provided, a default output dimension will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimension: Option<i64>,
    /// The data type of the output embeddings when feature available. If not provided, a default output data type will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dtype: Option<EmbeddingDtype>,
    /// The format of embeddings in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,
}

impl EmbeddingRequest {
    pub fn builder() -> EmbeddingRequestBuilder {
        <EmbeddingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbeddingRequestBuilder {
    model: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    input: Option<EmbeddingRequestInput>,
    output_dimension: Option<i64>,
    output_dtype: Option<EmbeddingDtype>,
    encoding_format: Option<EncodingFormat>,
}

impl EmbeddingRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn input(mut self, value: EmbeddingRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output_dimension(mut self, value: i64) -> Self {
        self.output_dimension = Some(value);
        self
    }

    pub fn output_dtype(mut self, value: EmbeddingDtype) -> Self {
        self.output_dtype = Some(value);
        self
    }

    pub fn encoding_format(mut self, value: EncodingFormat) -> Self {
        self.encoding_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmbeddingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](EmbeddingRequestBuilder::model)
    /// - [`input`](EmbeddingRequestBuilder::input)
    pub fn build(self) -> Result<EmbeddingRequest, BuildError> {
        Ok(EmbeddingRequest {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            metadata: self.metadata,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            output_dimension: self.output_dimension,
            output_dtype: self.output_dtype,
            encoding_format: self.encoding_format,
        })
    }
}
