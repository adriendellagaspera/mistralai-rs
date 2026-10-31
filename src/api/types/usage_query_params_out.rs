pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageQueryParamsOut {
    /// API key IDs to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<Vec<String>>,
    /// Inference regions to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_zone: Option<Vec<String>>,
    /// Customer-facing billing metric names to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_display_name: Option<Vec<String>>,
    /// Billing metric components to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group: Option<Vec<String>>,
    /// Billing metric identifiers to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_metric: Option<Vec<String>>,
    /// Customer scope derived from the authenticated Admin API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Exclusive end of the queried interval.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end: DateTime<FixedOffset>,
    /// Additional inclusion or exclusion filters applied to the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AdminUsageQueryFilter>>,
    /// Dimensions used to group the returned usage rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<UsageQueryParamsOutGroupByItem>>,
    /// Whether distinct API-call counts were requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nb_api_calls: Option<bool>,
    /// Applied row limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Applied row offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Organization IDs to include within the authenticated customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Vec<String>>,
    /// Service account IDs to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<Vec<String>>,
    /// Service tiers to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<Vec<String>>,
    /// Inclusive start of the queried interval.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start: DateTime<FixedOffset>,
    /// Time interval used to bucket grouped rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_bucket: Option<UsageQueryParamsOutTimeBucket>,
    /// Applied attribution filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untracked: Option<bool>,
    /// Usage event types to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<Vec<UsageType>>,
    /// User IDs to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Vec<String>>,
    /// Workspace IDs to include. Use null to include usage without a workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<Option<String>>>,
}

impl UsageQueryParamsOut {
    pub fn builder() -> UsageQueryParamsOutBuilder {
        <UsageQueryParamsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageQueryParamsOutBuilder {
    api_key_id: Option<Vec<String>>,
    api_zone: Option<Vec<String>>,
    billing_display_name: Option<Vec<String>>,
    billing_group: Option<Vec<String>>,
    billing_metric: Option<Vec<String>>,
    customer_id: Option<String>,
    end: Option<DateTime<FixedOffset>>,
    filters: Option<Vec<AdminUsageQueryFilter>>,
    group_by: Option<Vec<UsageQueryParamsOutGroupByItem>>,
    include_nb_api_calls: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    organization_id: Option<Vec<String>>,
    service_account_id: Option<Vec<String>>,
    service_tier: Option<Vec<String>>,
    start: Option<DateTime<FixedOffset>>,
    time_bucket: Option<UsageQueryParamsOutTimeBucket>,
    untracked: Option<bool>,
    usage_type: Option<Vec<UsageType>>,
    user_id: Option<Vec<String>>,
    workspace_ids: Option<Vec<Option<String>>>,
}

impl UsageQueryParamsOutBuilder {
    pub fn api_key_id(mut self, value: Vec<String>) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn api_zone(mut self, value: Vec<String>) -> Self {
        self.api_zone = Some(value);
        self
    }

    pub fn billing_display_name(mut self, value: Vec<String>) -> Self {
        self.billing_display_name = Some(value);
        self
    }

    pub fn billing_group(mut self, value: Vec<String>) -> Self {
        self.billing_group = Some(value);
        self
    }

    pub fn billing_metric(mut self, value: Vec<String>) -> Self {
        self.billing_metric = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<AdminUsageQueryFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<UsageQueryParamsOutGroupByItem>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn include_nb_api_calls(mut self, value: bool) -> Self {
        self.include_nb_api_calls = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn organization_id(mut self, value: Vec<String>) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn service_account_id(mut self, value: Vec<String>) -> Self {
        self.service_account_id = Some(value);
        self
    }

    pub fn service_tier(mut self, value: Vec<String>) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start = Some(value);
        self
    }

    pub fn time_bucket(mut self, value: UsageQueryParamsOutTimeBucket) -> Self {
        self.time_bucket = Some(value);
        self
    }

    pub fn untracked(mut self, value: bool) -> Self {
        self.untracked = Some(value);
        self
    }

    pub fn usage_type(mut self, value: Vec<UsageType>) -> Self {
        self.usage_type = Some(value);
        self
    }

    pub fn user_id(mut self, value: Vec<String>) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn workspace_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.workspace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageQueryParamsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](UsageQueryParamsOutBuilder::end)
    /// - [`start`](UsageQueryParamsOutBuilder::start)
    pub fn build(self) -> Result<UsageQueryParamsOut, BuildError> {
        Ok(UsageQueryParamsOut {
            api_key_id: self.api_key_id,
            api_zone: self.api_zone,
            billing_display_name: self.billing_display_name,
            billing_group: self.billing_group,
            billing_metric: self.billing_metric,
            customer_id: self.customer_id,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            filters: self.filters,
            group_by: self.group_by,
            include_nb_api_calls: self.include_nb_api_calls,
            limit: self.limit,
            offset: self.offset,
            organization_id: self.organization_id,
            service_account_id: self.service_account_id,
            service_tier: self.service_tier,
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
            time_bucket: self.time_bucket,
            untracked: self.untracked,
            usage_type: self.usage_type,
            user_id: self.user_id,
            workspace_ids: self.workspace_ids,
        })
    }
}
