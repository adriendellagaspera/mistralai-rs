pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageLimits {
    /// Whether the monthly usage limit has been reached.
    #[serde(default)]
    pub monthly_limit_reached: bool,
    /// Whether no monthly usage limit is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_monthly_limit: Option<bool>,
    /// Total current usage counted against the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<f64>,
    /// Current usage counted against the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<f64>,
    /// Current usage limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<f64>,
    /// Current Vibe usage counted against the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vibe_usage: Option<f64>,
}

impl UsageLimits {
    pub fn builder() -> UsageLimitsBuilder {
        <UsageLimitsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageLimitsBuilder {
    monthly_limit_reached: Option<bool>,
    no_monthly_limit: Option<bool>,
    total_usage: Option<f64>,
    usage: Option<f64>,
    usage_limit: Option<f64>,
    vibe_usage: Option<f64>,
}

impl UsageLimitsBuilder {
    pub fn monthly_limit_reached(mut self, value: bool) -> Self {
        self.monthly_limit_reached = Some(value);
        self
    }

    pub fn no_monthly_limit(mut self, value: bool) -> Self {
        self.no_monthly_limit = Some(value);
        self
    }

    pub fn total_usage(mut self, value: f64) -> Self {
        self.total_usage = Some(value);
        self
    }

    pub fn usage(mut self, value: f64) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn usage_limit(mut self, value: f64) -> Self {
        self.usage_limit = Some(value);
        self
    }

    pub fn vibe_usage(mut self, value: f64) -> Self {
        self.vibe_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageLimits`].
    /// This method will fail if any of the following fields are not set:
    /// - [`monthly_limit_reached`](UsageLimitsBuilder::monthly_limit_reached)
    pub fn build(self) -> Result<UsageLimits, BuildError> {
        Ok(UsageLimits {
            monthly_limit_reached: self
                .monthly_limit_reached
                .ok_or_else(|| BuildError::missing_field("monthly_limit_reached"))?,
            no_monthly_limit: self.no_monthly_limit,
            total_usage: self.total_usage,
            usage: self.usage,
            usage_limit: self.usage_limit,
            vibe_usage: self.vibe_usage,
        })
    }
}
