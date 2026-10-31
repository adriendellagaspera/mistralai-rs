pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageQueryRowOut {
    /// API key ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
    /// Inference region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_zone: Option<String>,
    /// Customer-facing billing metric name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_display_name: Option<String>,
    /// Billing metric component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group: Option<String>,
    /// Billing metric identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_metric: Option<String>,
    /// Product source of the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_source: Option<String>,
    /// Type of call within its source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_type: Option<String>,
    /// Number of usage records in this row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Customer ID for this row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Customer tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tier: Option<String>,
    /// Number of distinct API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_api_calls: Option<i64>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Service account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<String>,
    /// Service tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Start of the UTC time bucket, expressed using the request start timestamp's UTC offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_bucket: Option<DateTime<FixedOffset>>,
    /// Timestamp of an unbucketed usage row, expressed using the request start timestamp's UTC offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Whether attribution was unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untracked: Option<bool>,
    /// Usage event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Total usage value for this row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Workspace ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl UsageQueryRowOut {
    pub fn builder() -> UsageQueryRowOutBuilder {
        <UsageQueryRowOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageQueryRowOutBuilder {
    api_key_id: Option<String>,
    api_zone: Option<String>,
    billing_display_name: Option<String>,
    billing_group: Option<String>,
    billing_metric: Option<String>,
    call_source: Option<String>,
    call_type: Option<String>,
    count: Option<i64>,
    customer_id: Option<String>,
    customer_tier: Option<String>,
    nb_api_calls: Option<i64>,
    organization_id: Option<String>,
    service_account_id: Option<String>,
    service_tier: Option<String>,
    time_bucket: Option<DateTime<FixedOffset>>,
    timestamp: Option<DateTime<FixedOffset>>,
    untracked: Option<bool>,
    usage_type: Option<String>,
    user_id: Option<String>,
    value: Option<i64>,
    workspace_id: Option<String>,
}

impl UsageQueryRowOutBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn api_zone(mut self, value: impl Into<String>) -> Self {
        self.api_zone = Some(value.into());
        self
    }

    pub fn billing_display_name(mut self, value: impl Into<String>) -> Self {
        self.billing_display_name = Some(value.into());
        self
    }

    pub fn billing_group(mut self, value: impl Into<String>) -> Self {
        self.billing_group = Some(value.into());
        self
    }

    pub fn billing_metric(mut self, value: impl Into<String>) -> Self {
        self.billing_metric = Some(value.into());
        self
    }

    pub fn call_source(mut self, value: impl Into<String>) -> Self {
        self.call_source = Some(value.into());
        self
    }

    pub fn call_type(mut self, value: impl Into<String>) -> Self {
        self.call_type = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn customer_tier(mut self, value: impl Into<String>) -> Self {
        self.customer_tier = Some(value.into());
        self
    }

    pub fn nb_api_calls(mut self, value: i64) -> Self {
        self.nb_api_calls = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn service_account_id(mut self, value: impl Into<String>) -> Self {
        self.service_account_id = Some(value.into());
        self
    }

    pub fn service_tier(mut self, value: impl Into<String>) -> Self {
        self.service_tier = Some(value.into());
        self
    }

    pub fn time_bucket(mut self, value: DateTime<FixedOffset>) -> Self {
        self.time_bucket = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn untracked(mut self, value: bool) -> Self {
        self.untracked = Some(value);
        self
    }

    pub fn usage_type(mut self, value: impl Into<String>) -> Self {
        self.usage_type = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsageQueryRowOut`].
    pub fn build(self) -> Result<UsageQueryRowOut, BuildError> {
        Ok(UsageQueryRowOut {
            api_key_id: self.api_key_id,
            api_zone: self.api_zone,
            billing_display_name: self.billing_display_name,
            billing_group: self.billing_group,
            billing_metric: self.billing_metric,
            call_source: self.call_source,
            call_type: self.call_type,
            count: self.count,
            customer_id: self.customer_id,
            customer_tier: self.customer_tier,
            nb_api_calls: self.nb_api_calls,
            organization_id: self.organization_id,
            service_account_id: self.service_account_id,
            service_tier: self.service_tier,
            time_bucket: self.time_bucket,
            timestamp: self.timestamp,
            untracked: self.untracked,
            usage_type: self.usage_type,
            user_id: self.user_id,
            value: self.value,
            workspace_id: self.workspace_id,
        })
    }
}
