pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LimitsContext {
    /// Completion usage and rate limits.
    #[serde(default)]
    pub completion: UsageLimits,
    /// Currency used for usage and limit amounts.
    #[serde(default)]
    pub currency: String,
    /// Whether the latest payment attempt failed.
    #[serde(default)]
    pub last_payment_failure: bool,
    /// Whether payment failure protection is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_failure_protection: Option<bool>,
}

impl LimitsContext {
    pub fn builder() -> LimitsContextBuilder {
        <LimitsContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LimitsContextBuilder {
    completion: Option<UsageLimits>,
    currency: Option<String>,
    last_payment_failure: Option<bool>,
    last_payment_failure_protection: Option<bool>,
}

impl LimitsContextBuilder {
    pub fn completion(mut self, value: UsageLimits) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn last_payment_failure(mut self, value: bool) -> Self {
        self.last_payment_failure = Some(value);
        self
    }

    pub fn last_payment_failure_protection(mut self, value: bool) -> Self {
        self.last_payment_failure_protection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LimitsContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completion`](LimitsContextBuilder::completion)
    /// - [`currency`](LimitsContextBuilder::currency)
    /// - [`last_payment_failure`](LimitsContextBuilder::last_payment_failure)
    pub fn build(self) -> Result<LimitsContext, BuildError> {
        Ok(LimitsContext {
            completion: self
                .completion
                .ok_or_else(|| BuildError::missing_field("completion"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            last_payment_failure: self
                .last_payment_failure
                .ok_or_else(|| BuildError::missing_field("last_payment_failure"))?,
            last_payment_failure_protection: self.last_payment_failure_protection,
        })
    }
}
