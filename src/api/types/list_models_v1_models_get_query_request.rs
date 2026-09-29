pub use crate::prelude::*;

/// Query parameters for list_models_v1_models_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListModelsV1ModelsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl ListModelsV1ModelsGetQueryRequest {
    pub fn builder() -> ListModelsV1ModelsGetQueryRequestBuilder {
        <ListModelsV1ModelsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListModelsV1ModelsGetQueryRequestBuilder {
    provider: Option<String>,
    model: Option<String>,
}

impl ListModelsV1ModelsGetQueryRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListModelsV1ModelsGetQueryRequest`].
    pub fn build(self) -> Result<ListModelsV1ModelsGetQueryRequest, BuildError> {
        Ok(ListModelsV1ModelsGetQueryRequest {
            provider: self.provider,
            model: self.model,
        })
    }
}
