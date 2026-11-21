pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Webhook {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub http_method: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub organization_id: String,
    pub owner_type: TriggersOwnerType,
    pub provider: WebhookProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_encoding: Option<WebhookSignatureEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_extractor_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_hash_algorithm: Option<WebhookSignatureHashAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_header_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_payload_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_status_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_extractor_regex: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub url: String,
    /// Token the provider echoes back when verifying the URL (WhatsApp: the Verify token)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_verification_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period_second: Option<i64>,
    #[serde(default)]
    pub workspace_id: String,
}

impl Webhook {
    pub fn builder() -> WebhookBuilder {
        <WebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    event_id_pattern: Option<String>,
    event_type_path: Option<String>,
    event_type_pattern: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    http_method: Option<String>,
    id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    owner_type: Option<TriggersOwnerType>,
    provider: Option<WebhookProvider>,
    signature_encoding: Option<WebhookSignatureEncoding>,
    signature_extractor_regex: Option<String>,
    signature_hash_algorithm: Option<WebhookSignatureHashAlgorithm>,
    signature_header_name: Option<String>,
    signature_payload_pattern: Option<String>,
    success_status_code: Option<i64>,
    timestamp_extractor_regex: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
    url_verification_token: Option<String>,
    url_verified: Option<bool>,
    validity_period_second: Option<i64>,
    workspace_id: Option<String>,
}

impl WebhookBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn event_id_pattern(mut self, value: impl Into<String>) -> Self {
        self.event_id_pattern = Some(value.into());
        self
    }

    pub fn event_type_path(mut self, value: impl Into<String>) -> Self {
        self.event_type_path = Some(value.into());
        self
    }

    pub fn event_type_pattern(mut self, value: impl Into<String>) -> Self {
        self.event_type_pattern = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn http_method(mut self, value: impl Into<String>) -> Self {
        self.http_method = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: TriggersOwnerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn provider(mut self, value: WebhookProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn signature_encoding(mut self, value: WebhookSignatureEncoding) -> Self {
        self.signature_encoding = Some(value);
        self
    }

    pub fn signature_extractor_regex(mut self, value: impl Into<String>) -> Self {
        self.signature_extractor_regex = Some(value.into());
        self
    }

    pub fn signature_hash_algorithm(mut self, value: WebhookSignatureHashAlgorithm) -> Self {
        self.signature_hash_algorithm = Some(value);
        self
    }

    pub fn signature_header_name(mut self, value: impl Into<String>) -> Self {
        self.signature_header_name = Some(value.into());
        self
    }

    pub fn signature_payload_pattern(mut self, value: impl Into<String>) -> Self {
        self.signature_payload_pattern = Some(value.into());
        self
    }

    pub fn success_status_code(mut self, value: i64) -> Self {
        self.success_status_code = Some(value);
        self
    }

    pub fn timestamp_extractor_regex(mut self, value: impl Into<String>) -> Self {
        self.timestamp_extractor_regex = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn url_verification_token(mut self, value: impl Into<String>) -> Self {
        self.url_verification_token = Some(value.into());
        self
    }

    pub fn url_verified(mut self, value: bool) -> Self {
        self.url_verified = Some(value);
        self
    }

    pub fn validity_period_second(mut self, value: i64) -> Self {
        self.validity_period_second = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Webhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](WebhookBuilder::created_at)
    /// - [`creator_id`](WebhookBuilder::creator_id)
    /// - [`http_method`](WebhookBuilder::http_method)
    /// - [`id`](WebhookBuilder::id)
    /// - [`name`](WebhookBuilder::name)
    /// - [`organization_id`](WebhookBuilder::organization_id)
    /// - [`owner_type`](WebhookBuilder::owner_type)
    /// - [`provider`](WebhookBuilder::provider)
    /// - [`updated_at`](WebhookBuilder::updated_at)
    /// - [`url`](WebhookBuilder::url)
    /// - [`workspace_id`](WebhookBuilder::workspace_id)
    pub fn build(self) -> Result<Webhook, BuildError> {
        Ok(Webhook {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            event_id_pattern: self.event_id_pattern,
            event_type_path: self.event_type_path,
            event_type_pattern: self.event_type_pattern,
            expires_at: self.expires_at,
            http_method: self
                .http_method
                .ok_or_else(|| BuildError::missing_field("http_method"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            signature_encoding: self.signature_encoding,
            signature_extractor_regex: self.signature_extractor_regex,
            signature_hash_algorithm: self.signature_hash_algorithm,
            signature_header_name: self.signature_header_name,
            signature_payload_pattern: self.signature_payload_pattern,
            success_status_code: self.success_status_code,
            timestamp_extractor_regex: self.timestamp_extractor_regex,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            url_verification_token: self.url_verification_token,
            url_verified: self.url_verified,
            validity_period_second: self.validity_period_second,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
