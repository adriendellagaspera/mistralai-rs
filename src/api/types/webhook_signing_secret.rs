pub use crate::prelude::*;

/// Signing secret for a Notion webhook, returned to complete URL verification
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookSigningSecret {
    #[serde(default)]
    pub signing_secret: String,
}

impl WebhookSigningSecret {
    pub fn builder() -> WebhookSigningSecretBuilder {
        <WebhookSigningSecretBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSigningSecretBuilder {
    signing_secret: Option<String>,
}

impl WebhookSigningSecretBuilder {
    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookSigningSecret`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signing_secret`](WebhookSigningSecretBuilder::signing_secret)
    pub fn build(self) -> Result<WebhookSigningSecret, BuildError> {
        Ok(WebhookSigningSecret {
            signing_secret: self
                .signing_secret
                .ok_or_else(|| BuildError::missing_field("signing_secret"))?,
        })
    }
}
