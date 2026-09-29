pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiKeysExtendedOut {
    /// API keys for the Organization.
    #[serde(default)]
    pub keys: Vec<ApiKeyExtendedOut>,
}

impl ApiKeysExtendedOut {
    pub fn builder() -> ApiKeysExtendedOutBuilder {
        <ApiKeysExtendedOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysExtendedOutBuilder {
    keys: Option<Vec<ApiKeyExtendedOut>>,
}

impl ApiKeysExtendedOutBuilder {
    pub fn keys(mut self, value: Vec<ApiKeyExtendedOut>) -> Self {
        self.keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysExtendedOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keys`](ApiKeysExtendedOutBuilder::keys)
    pub fn build(self) -> Result<ApiKeysExtendedOut, BuildError> {
        Ok(ApiKeysExtendedOut {
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
        })
    }
}
