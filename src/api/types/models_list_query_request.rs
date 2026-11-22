pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModelsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl ModelsListQueryRequest {
    pub fn builder() -> ModelsListQueryRequestBuilder {
        <ModelsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelsListQueryRequestBuilder {
    provider: Option<String>,
    model: Option<String>,
}

impl ModelsListQueryRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ModelsListQueryRequest`].
    pub fn build(self) -> Result<ModelsListQueryRequest, BuildError> {
        Ok(ModelsListQueryRequest {
            provider: self.provider,
            model: self.model,
        })
    }
}
