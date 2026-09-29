pub use crate::prelude::*;

/// Value of a connector-wide header. ``value`` is plaintext in memory so create
/// round-trips and encryption-at-rest keep the real value; secrets are redacted only
/// on JSON serialization (API responses).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GlobalHeaderValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[serde(default)]
    pub value: String,
}

impl GlobalHeaderValue {
    pub fn builder() -> GlobalHeaderValueBuilder {
        <GlobalHeaderValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GlobalHeaderValueBuilder {
    is_secret: Option<bool>,
    value: Option<String>,
}

impl GlobalHeaderValueBuilder {
    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GlobalHeaderValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](GlobalHeaderValueBuilder::value)
    pub fn build(self) -> Result<GlobalHeaderValue, BuildError> {
        Ok(GlobalHeaderValue {
            is_secret: self.is_secret,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
