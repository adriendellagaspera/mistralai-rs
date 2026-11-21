pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OAuth2TokenAuth {
    #[serde(default)]
    pub value: String,
}

impl OAuth2TokenAuth {
    pub fn builder() -> OAuth2TokenAuthBuilder {
        <OAuth2TokenAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuth2TokenAuthBuilder {
    value: Option<String>,
}

impl OAuth2TokenAuthBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OAuth2TokenAuth`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](OAuth2TokenAuthBuilder::value)
    pub fn build(self) -> Result<OAuth2TokenAuth, BuildError> {
        Ok(OAuth2TokenAuth {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
