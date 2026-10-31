pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientSecret {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub value: String,
}

impl ClientSecret {
    pub fn builder() -> ClientSecretBuilder {
        <ClientSecretBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientSecretBuilder {
    expires_at: Option<DateTime<FixedOffset>>,
    value: Option<String>,
}

impl ClientSecretBuilder {
    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientSecret`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](ClientSecretBuilder::expires_at)
    /// - [`value`](ClientSecretBuilder::value)
    pub fn build(self) -> Result<ClientSecret, BuildError> {
        Ok(ClientSecret {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
