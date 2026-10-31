pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GoalStatusFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_name: Option<String>,
    #[serde(default)]
    pub values: Vec<GoalStatus>,
}

impl GoalStatusFilter {
    pub fn builder() -> GoalStatusFilterBuilder {
        <GoalStatusFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoalStatusFilterBuilder {
    evaluator_name: Option<String>,
    values: Option<Vec<GoalStatus>>,
}

impl GoalStatusFilterBuilder {
    pub fn evaluator_name(mut self, value: impl Into<String>) -> Self {
        self.evaluator_name = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<GoalStatus>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoalStatusFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`values`](GoalStatusFilterBuilder::values)
    pub fn build(self) -> Result<GoalStatusFilter, BuildError> {
        Ok(GoalStatusFilter {
            evaluator_name: self.evaluator_name,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
