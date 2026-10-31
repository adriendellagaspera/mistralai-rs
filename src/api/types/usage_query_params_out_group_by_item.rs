pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageQueryParamsOutGroupByItem {
    ApiZone,
    ServiceTier,
    CustomerId,
    UsageType,
    BillingMetric,
    BillingGroup,
    BillingDisplayName,
    WorkspaceId,
    OrganizationId,
    UserId,
    ServiceAccountId,
    ApiKeyId,
    Untracked,
    CallSource,
    CallType,
    CustomerTier,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsageQueryParamsOutGroupByItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApiZone => serializer.serialize_str("api_zone"),
            Self::ServiceTier => serializer.serialize_str("service_tier"),
            Self::CustomerId => serializer.serialize_str("customer_id"),
            Self::UsageType => serializer.serialize_str("usage_type"),
            Self::BillingMetric => serializer.serialize_str("billing_metric"),
            Self::BillingGroup => serializer.serialize_str("billing_group"),
            Self::BillingDisplayName => serializer.serialize_str("billing_display_name"),
            Self::WorkspaceId => serializer.serialize_str("workspace_id"),
            Self::OrganizationId => serializer.serialize_str("organization_id"),
            Self::UserId => serializer.serialize_str("user_id"),
            Self::ServiceAccountId => serializer.serialize_str("service_account_id"),
            Self::ApiKeyId => serializer.serialize_str("api_key_id"),
            Self::Untracked => serializer.serialize_str("untracked"),
            Self::CallSource => serializer.serialize_str("call_source"),
            Self::CallType => serializer.serialize_str("call_type"),
            Self::CustomerTier => serializer.serialize_str("customer_tier"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsageQueryParamsOutGroupByItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api_zone" => Ok(Self::ApiZone),
            "service_tier" => Ok(Self::ServiceTier),
            "customer_id" => Ok(Self::CustomerId),
            "usage_type" => Ok(Self::UsageType),
            "billing_metric" => Ok(Self::BillingMetric),
            "billing_group" => Ok(Self::BillingGroup),
            "billing_display_name" => Ok(Self::BillingDisplayName),
            "workspace_id" => Ok(Self::WorkspaceId),
            "organization_id" => Ok(Self::OrganizationId),
            "user_id" => Ok(Self::UserId),
            "service_account_id" => Ok(Self::ServiceAccountId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "untracked" => Ok(Self::Untracked),
            "call_source" => Ok(Self::CallSource),
            "call_type" => Ok(Self::CallType),
            "customer_tier" => Ok(Self::CustomerTier),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsageQueryParamsOutGroupByItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiZone => write!(f, "api_zone"),
            Self::ServiceTier => write!(f, "service_tier"),
            Self::CustomerId => write!(f, "customer_id"),
            Self::UsageType => write!(f, "usage_type"),
            Self::BillingMetric => write!(f, "billing_metric"),
            Self::BillingGroup => write!(f, "billing_group"),
            Self::BillingDisplayName => write!(f, "billing_display_name"),
            Self::WorkspaceId => write!(f, "workspace_id"),
            Self::OrganizationId => write!(f, "organization_id"),
            Self::UserId => write!(f, "user_id"),
            Self::ServiceAccountId => write!(f, "service_account_id"),
            Self::ApiKeyId => write!(f, "api_key_id"),
            Self::Untracked => write!(f, "untracked"),
            Self::CallSource => write!(f, "call_source"),
            Self::CallType => write!(f, "call_type"),
            Self::CustomerTier => write!(f, "customer_tier"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
