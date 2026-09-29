pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiKeyOut {
    /// Per-action availability for this API key (e.g. whether it can be rotated or deleted).
    #[serde(default)]
    pub actions: ApiKeyActions,
    /// Time when the API key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Date when the API key expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
    /// Masked API key value for display.
    #[serde(default)]
    pub hidden_key: String,
    /// Plaintext API key value. Only returned at creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// API key ID.
    #[serde(default)]
    pub key_id: String,
    /// Name of the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ApiKeyOut {
    pub fn builder() -> ApiKeyOutBuilder {
        <ApiKeyOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyOutBuilder {
    actions: Option<ApiKeyActions>,
    created_at: Option<DateTime<FixedOffset>>,
    expiration_date: Option<NaiveDate>,
    hidden_key: Option<String>,
    key: Option<String>,
    key_id: Option<String>,
    name: Option<String>,
}

impl ApiKeyOutBuilder {
    pub fn actions(mut self, value: ApiKeyActions) -> Self {
        self.actions = Some(value);
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

    pub fn hidden_key(mut self, value: impl Into<String>) -> Self {
        self.hidden_key = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](ApiKeyOutBuilder::actions)
    /// - [`hidden_key`](ApiKeyOutBuilder::hidden_key)
    /// - [`key_id`](ApiKeyOutBuilder::key_id)
    pub fn build(self) -> Result<ApiKeyOut, BuildError> {
        Ok(ApiKeyOut {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            created_at: self.created_at,
            expiration_date: self.expiration_date,
            hidden_key: self
                .hidden_key
                .ok_or_else(|| BuildError::missing_field("hidden_key"))?,
            key: self.key,
            key_id: self
                .key_id
                .ok_or_else(|| BuildError::missing_field("key_id"))?,
            name: self.name,
        })
    }
}
