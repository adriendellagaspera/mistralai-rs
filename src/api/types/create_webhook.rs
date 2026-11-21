pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateWebhook {
    /// Encryption key used to decrypt webhook payloads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// Pattern for a unique event identifier used to deduplicate deliveries, with {body.<path>} and {headers.<Header-Name>} placeholders, eg {body.id} or {body.event_id}.{headers.X-Delivery-Id}. Defaults to a hash of the request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_pattern: Option<String>,
    /// Pattern for the event type, with {body.<path>} and {headers.<Header-Name>} placeholders, eg {body.type} or {headers.X-GitHub-Event}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Human-readable name for this webhook
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<TriggersOwnerType>,
    pub provider: WebhookProvider,
    /// Signing secret provided by the third-party provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_encoding: Option<WebhookSignatureEncoding>,
    /// Regex to extract signature, eg sha256=(.*)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_extractor_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_hash_algorithm: Option<WebhookSignatureHashAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_header_name: Option<String>,
    /// Pattern for signature payload, eg {body} or {timestamp}.{body}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_payload_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_status_code: Option<i64>,
    /// Regex to extract the timestamp from the signature header, used as {timestamp}, eg t=([^,]+)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_extractor_regex: Option<String>,
    /// Token the provider echoes back when verifying the URL. Required for WhatsApp (paste the same value in the Verify token field of the Meta App Dashboard), forbidden for other providers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_verification_token: Option<String>,
    /// Maximum allowed drift in seconds between the signed timestamp and reception time, eg 300
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period_second: Option<i64>,
}

impl CreateWebhook {
    pub fn builder() -> CreateWebhookBuilder {
        <CreateWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookBuilder {
    encryption_key: Option<String>,
    event_id_pattern: Option<String>,
    event_type_pattern: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    owner_type: Option<TriggersOwnerType>,
    provider: Option<WebhookProvider>,
    secret: Option<String>,
    signature_encoding: Option<WebhookSignatureEncoding>,
    signature_extractor_regex: Option<String>,
    signature_hash_algorithm: Option<WebhookSignatureHashAlgorithm>,
    signature_header_name: Option<String>,
    signature_payload_pattern: Option<String>,
    success_status_code: Option<i64>,
    timestamp_extractor_regex: Option<String>,
    url_verification_token: Option<String>,
    validity_period_second: Option<i64>,
}

impl CreateWebhookBuilder {
    pub fn encryption_key(mut self, value: impl Into<String>) -> Self {
        self.encryption_key = Some(value.into());
        self
    }

    pub fn event_id_pattern(mut self, value: impl Into<String>) -> Self {
        self.event_id_pattern = Some(value.into());
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
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

    pub fn url_verification_token(mut self, value: impl Into<String>) -> Self {
        self.url_verification_token = Some(value.into());
        self
    }

    pub fn validity_period_second(mut self, value: i64) -> Self {
        self.validity_period_second = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateWebhookBuilder::name)
    /// - [`provider`](CreateWebhookBuilder::provider)
    pub fn build(self) -> Result<CreateWebhook, BuildError> {
        Ok(CreateWebhook {
            encryption_key: self.encryption_key,
            event_id_pattern: self.event_id_pattern,
            event_type_pattern: self.event_type_pattern,
            expires_at: self.expires_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            owner_type: self.owner_type,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            secret: self.secret,
            signature_encoding: self.signature_encoding,
            signature_extractor_regex: self.signature_extractor_regex,
            signature_hash_algorithm: self.signature_hash_algorithm,
            signature_header_name: self.signature_header_name,
            signature_payload_pattern: self.signature_payload_pattern,
            success_status_code: self.success_status_code,
            timestamp_extractor_regex: self.timestamp_extractor_regex,
            url_verification_token: self.url_verification_token,
            validity_period_second: self.validity_period_second,
        })
    }
}
