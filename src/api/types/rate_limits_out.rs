pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RateLimitsOut {
    /// Maximum API requests allowed per second.
    #[serde(default)]
    pub requests_per_second: i64,
    /// Token limits for each model.
    #[serde(default)]
    pub tokens_limits_by_model: HashMap<String, TokenLimitsByModel>,
}

impl RateLimitsOut {
    pub fn builder() -> RateLimitsOutBuilder {
        <RateLimitsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RateLimitsOutBuilder {
    requests_per_second: Option<i64>,
    tokens_limits_by_model: Option<HashMap<String, TokenLimitsByModel>>,
}

impl RateLimitsOutBuilder {
    pub fn requests_per_second(mut self, value: i64) -> Self {
        self.requests_per_second = Some(value);
        self
    }

    pub fn tokens_limits_by_model(mut self, value: HashMap<String, TokenLimitsByModel>) -> Self {
        self.tokens_limits_by_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RateLimitsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests_per_second`](RateLimitsOutBuilder::requests_per_second)
    /// - [`tokens_limits_by_model`](RateLimitsOutBuilder::tokens_limits_by_model)
    pub fn build(self) -> Result<RateLimitsOut, BuildError> {
        Ok(RateLimitsOut {
            requests_per_second: self
                .requests_per_second
                .ok_or_else(|| BuildError::missing_field("requests_per_second"))?,
            tokens_limits_by_model: self
                .tokens_limits_by_model
                .ok_or_else(|| BuildError::missing_field("tokens_limits_by_model"))?,
        })
    }
}
