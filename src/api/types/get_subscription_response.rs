pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSubscriptionResponse {
    /// Whether the organization is on the Enterprise tier (org_tier == ENTERPRISE).
    #[serde(default)]
    pub enterprise: bool,
    /// Whether pay-as-you-go is active (an API subscription is active).
    #[serde(default)]
    pub payg_enabled: bool,
    /// Unified plan name. One of: FREE, INDIVIDUAL, EDU, TEAM. Reflects the active Mistral licence (chat subscription) or FREE when no subscription exists.
    #[serde(default)]
    pub plan_name: String,
    /// Whether pay-as-you-go is enabled for Vibe Code specifically.
    #[serde(default)]
    pub vibe_payg_enabled: bool,
}

impl GetSubscriptionResponse {
    pub fn builder() -> GetSubscriptionResponseBuilder {
        <GetSubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubscriptionResponseBuilder {
    enterprise: Option<bool>,
    payg_enabled: Option<bool>,
    plan_name: Option<String>,
    vibe_payg_enabled: Option<bool>,
}

impl GetSubscriptionResponseBuilder {
    pub fn enterprise(mut self, value: bool) -> Self {
        self.enterprise = Some(value);
        self
    }

    pub fn payg_enabled(mut self, value: bool) -> Self {
        self.payg_enabled = Some(value);
        self
    }

    pub fn plan_name(mut self, value: impl Into<String>) -> Self {
        self.plan_name = Some(value.into());
        self
    }

    pub fn vibe_payg_enabled(mut self, value: bool) -> Self {
        self.vibe_payg_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enterprise`](GetSubscriptionResponseBuilder::enterprise)
    /// - [`payg_enabled`](GetSubscriptionResponseBuilder::payg_enabled)
    /// - [`plan_name`](GetSubscriptionResponseBuilder::plan_name)
    /// - [`vibe_payg_enabled`](GetSubscriptionResponseBuilder::vibe_payg_enabled)
    pub fn build(self) -> Result<GetSubscriptionResponse, BuildError> {
        Ok(GetSubscriptionResponse {
            enterprise: self
                .enterprise
                .ok_or_else(|| BuildError::missing_field("enterprise"))?,
            payg_enabled: self
                .payg_enabled
                .ok_or_else(|| BuildError::missing_field("payg_enabled"))?,
            plan_name: self
                .plan_name
                .ok_or_else(|| BuildError::missing_field("plan_name"))?,
            vibe_payg_enabled: self
                .vibe_payg_enabled
                .ok_or_else(|| BuildError::missing_field("vibe_payg_enabled"))?,
        })
    }
}
