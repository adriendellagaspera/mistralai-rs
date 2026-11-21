pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyAuth {
    #[serde(default)]
    pub value: String,
}

impl ApiKeyAuth {
    pub fn builder() -> ApiKeyAuthBuilder {
        <ApiKeyAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyAuthBuilder {
    value: Option<String>,
}

impl ApiKeyAuthBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyAuth`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ApiKeyAuthBuilder::value)
    pub fn build(self) -> Result<ApiKeyAuth, BuildError> {
        Ok(ApiKeyAuth {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
