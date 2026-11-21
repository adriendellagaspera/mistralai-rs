pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetBudgetResponse {
    /// API budget usage details.
    #[serde(default)]
    pub api_budget: BudgetDetail,
    /// Vibe-specific budget usage details.
    #[serde(default)]
    pub vibe_budget: BudgetDetail,
}

impl GetBudgetResponse {
    pub fn builder() -> GetBudgetResponseBuilder {
        <GetBudgetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBudgetResponseBuilder {
    api_budget: Option<BudgetDetail>,
    vibe_budget: Option<BudgetDetail>,
}

impl GetBudgetResponseBuilder {
    pub fn api_budget(mut self, value: BudgetDetail) -> Self {
        self.api_budget = Some(value);
        self
    }

    pub fn vibe_budget(mut self, value: BudgetDetail) -> Self {
        self.vibe_budget = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBudgetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_budget`](GetBudgetResponseBuilder::api_budget)
    /// - [`vibe_budget`](GetBudgetResponseBuilder::vibe_budget)
    pub fn build(self) -> Result<GetBudgetResponse, BuildError> {
        Ok(GetBudgetResponse {
            api_budget: self
                .api_budget
                .ok_or_else(|| BuildError::missing_field("api_budget"))?,
            vibe_budget: self
                .vibe_budget
                .ok_or_else(|| BuildError::missing_field("vibe_budget"))?,
        })
    }
}
