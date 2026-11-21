pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BudgetDetail {
    /// ISO 4217 currency code of the budget (e.g. EUR, USD). The applicable budget is lazily materialized for the requested period, so this is only null when no budget could be derived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Initial budget allocated for the billing period, expressed in the budget currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_budget: f64,
    /// Whether pay-as-you-go overage is enabled for this budget type. Only meaningful for the Vibe budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payg_enabled: Option<bool>,
    /// Timestamp at which the requested billing period resets (start of the next period).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub reset_at: DateTime<FixedOffset>,
    /// Percentage of the budget consumed for the requested billing period (0-100).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub usage_percentage: f64,
}

impl BudgetDetail {
    pub fn builder() -> BudgetDetailBuilder {
        <BudgetDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BudgetDetailBuilder {
    currency: Option<String>,
    initial_budget: Option<f64>,
    payg_enabled: Option<bool>,
    reset_at: Option<DateTime<FixedOffset>>,
    usage_percentage: Option<f64>,
}

impl BudgetDetailBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn initial_budget(mut self, value: f64) -> Self {
        self.initial_budget = Some(value);
        self
    }

    pub fn payg_enabled(mut self, value: bool) -> Self {
        self.payg_enabled = Some(value);
        self
    }

    pub fn reset_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.reset_at = Some(value);
        self
    }

    pub fn usage_percentage(mut self, value: f64) -> Self {
        self.usage_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BudgetDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`initial_budget`](BudgetDetailBuilder::initial_budget)
    /// - [`reset_at`](BudgetDetailBuilder::reset_at)
    /// - [`usage_percentage`](BudgetDetailBuilder::usage_percentage)
    pub fn build(self) -> Result<BudgetDetail, BuildError> {
        Ok(BudgetDetail {
            currency: self.currency,
            initial_budget: self
                .initial_budget
                .ok_or_else(|| BuildError::missing_field("initial_budget"))?,
            payg_enabled: self.payg_enabled,
            reset_at: self
                .reset_at
                .ok_or_else(|| BuildError::missing_field("reset_at"))?,
            usage_percentage: self
                .usage_percentage
                .ok_or_else(|| BuildError::missing_field("usage_percentage"))?,
        })
    }
}
