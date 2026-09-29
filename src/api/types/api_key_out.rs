pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiKeyOut {
    /// API key ID.
    #[serde(default)]
    pub key_id: String,
    /// Plaintext API key value. Only returned at creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Name of the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Masked API key value for display.
    #[serde(default)]
    pub hidden_key: String,
    /// Time when the API key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Date when the API key expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
    /// Per-action availability for this API key (e.g. whether it can be rotated or deleted).
    #[serde(default)]
    pub actions: ApiKeyActions,
}

impl ApiKeyOut {
    pub fn builder() -> ApiKeyOutBuilder {
        <ApiKeyOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyOutBuilder {
    key_id: Option<String>,
    key: Option<String>,
    name: Option<String>,
    hidden_key: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    expiration_date: Option<NaiveDate>,
    actions: Option<ApiKeyActions>,
}

impl ApiKeyOutBuilder {
    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn hidden_key(mut self, value: impl Into<String>) -> Self {
        self.hidden_key = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: NaiveDate) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn actions(mut self, value: ApiKeyActions) -> Self {
        self.actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key_id`](ApiKeyOutBuilder::key_id)
    /// - [`hidden_key`](ApiKeyOutBuilder::hidden_key)
    /// - [`actions`](ApiKeyOutBuilder::actions)
    pub fn build(self) -> Result<ApiKeyOut, BuildError> {
        Ok(ApiKeyOut {
            key_id: self
                .key_id
                .ok_or_else(|| BuildError::missing_field("key_id"))?,
            key: self.key,
            name: self.name,
            hidden_key: self
                .hidden_key
                .ok_or_else(|| BuildError::missing_field("hidden_key"))?,
            created_at: self.created_at,
            expiration_date: self.expiration_date,
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
        })
    }
}
