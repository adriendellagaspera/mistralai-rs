pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GoalResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    #[serde(default)]
    pub results: Vec<GoalResultItem>,
}

impl GoalResult {
    pub fn builder() -> GoalResultBuilder {
        <GoalResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoalResultBuilder {
    passed: Option<bool>,
    results: Option<Vec<GoalResultItem>>,
}

impl GoalResultBuilder {
    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<GoalResultItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoalResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](GoalResultBuilder::results)
    pub fn build(self) -> Result<GoalResult, BuildError> {
        Ok(GoalResult {
            passed: self.passed,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
