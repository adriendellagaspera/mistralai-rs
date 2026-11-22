pub use crate::prelude::*;

/// Query parameters for get_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaPromptsGetVersionQueryRequest {
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl BetaPromptsGetVersionQueryRequest {
    pub fn builder() -> BetaPromptsGetVersionQueryRequestBuilder {
        <BetaPromptsGetVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaPromptsGetVersionQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
}

impl BetaPromptsGetVersionQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaPromptsGetVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](BetaPromptsGetVersionQueryRequestBuilder::fields)
    pub fn build(self) -> Result<BetaPromptsGetVersionQueryRequest, BuildError> {
        Ok(BetaPromptsGetVersionQueryRequest {
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
