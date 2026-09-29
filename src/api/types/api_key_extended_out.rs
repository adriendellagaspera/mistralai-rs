pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiKeyExtendedOut {
    /// Per-action availability for this API key (e.g. whether it can be rotated or deleted).
    #[serde(default)]
    pub actions: ApiKeyActions,
    /// Whether you can delete this API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    /// Time when the API key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// User or API key that created this API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
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
    /// Date when the API key was last used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<NaiveDate>,
    /// Name of the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product the API key belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ApiKeyProduct>,
    /// Workspace ID for the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Name of the Workspace for the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

impl ApiKeyExtendedOut {
    pub fn builder() -> ApiKeyExtendedOutBuilder {
        <ApiKeyExtendedOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyExtendedOutBuilder {
    actions: Option<ApiKeyActions>,
    can_delete: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    expiration_date: Option<NaiveDate>,
    hidden_key: Option<String>,
    key: Option<String>,
    key_id: Option<String>,
    last_used: Option<NaiveDate>,
    name: Option<String>,
    product: Option<ApiKeyProduct>,
    workspace_id: Option<String>,
    workspace_name: Option<String>,
}

impl ApiKeyExtendedOutBuilder {
    pub fn actions(mut self, value: ApiKeyActions) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn can_delete(mut self, value: bool) -> Self {
        self.can_delete = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
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

    pub fn last_used(mut self, value: NaiveDate) -> Self {
        self.last_used = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn product(mut self, value: ApiKeyProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyExtendedOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](ApiKeyExtendedOutBuilder::actions)
    /// - [`hidden_key`](ApiKeyExtendedOutBuilder::hidden_key)
    /// - [`key_id`](ApiKeyExtendedOutBuilder::key_id)
    pub fn build(self) -> Result<ApiKeyExtendedOut, BuildError> {
        Ok(ApiKeyExtendedOut {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            can_delete: self.can_delete,
            created_at: self.created_at,
            created_by: self.created_by,
            expiration_date: self.expiration_date,
            hidden_key: self
                .hidden_key
                .ok_or_else(|| BuildError::missing_field("hidden_key"))?,
            key: self.key,
            key_id: self
                .key_id
                .ok_or_else(|| BuildError::missing_field("key_id"))?,
            last_used: self.last_used,
            name: self.name,
            product: self.product,
            workspace_id: self.workspace_id,
            workspace_name: self.workspace_name,
        })
    }
}
