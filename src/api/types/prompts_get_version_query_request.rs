pub use crate::prelude::*;

/// Query parameters for prompts_get_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptsGetVersionQueryRequest {
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl PromptsGetVersionQueryRequest {
    pub fn builder() -> PromptsGetVersionQueryRequestBuilder {
        <PromptsGetVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptsGetVersionQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
}

impl PromptsGetVersionQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptsGetVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](PromptsGetVersionQueryRequestBuilder::fields)
    pub fn build(self) -> Result<PromptsGetVersionQueryRequest, BuildError> {
        Ok(PromptsGetVersionQueryRequest {
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
