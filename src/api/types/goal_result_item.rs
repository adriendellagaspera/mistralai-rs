pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GoalResultItem {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub actual: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<GoalResultItemDirection>,
    #[serde(default)]
    pub metric: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_min: Option<f64>,
}

impl GoalResultItem {
    pub fn builder() -> GoalResultItemBuilder {
        <GoalResultItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoalResultItemBuilder {
    actual: Option<f64>,
    direction: Option<GoalResultItemDirection>,
    metric: Option<String>,
    operator: Option<String>,
    passed: Option<bool>,
    target: Option<f64>,
    target_max: Option<f64>,
    target_min: Option<f64>,
}

impl GoalResultItemBuilder {
    pub fn actual(mut self, value: f64) -> Self {
        self.actual = Some(value);
        self
    }

    pub fn direction(mut self, value: GoalResultItemDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn metric(mut self, value: impl Into<String>) -> Self {
        self.metric = Some(value.into());
        self
    }

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn target(mut self, value: f64) -> Self {
        self.target = Some(value);
        self
    }

    pub fn target_max(mut self, value: f64) -> Self {
        self.target_max = Some(value);
        self
    }

    pub fn target_min(mut self, value: f64) -> Self {
        self.target_min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoalResultItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actual`](GoalResultItemBuilder::actual)
    /// - [`metric`](GoalResultItemBuilder::metric)
    /// - [`operator`](GoalResultItemBuilder::operator)
    /// - [`passed`](GoalResultItemBuilder::passed)
    pub fn build(self) -> Result<GoalResultItem, BuildError> {
        Ok(GoalResultItem {
            actual: self
                .actual
                .ok_or_else(|| BuildError::missing_field("actual"))?,
            direction: self.direction,
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            passed: self
                .passed
                .ok_or_else(|| BuildError::missing_field("passed"))?,
            target: self.target,
            target_max: self.target_max,
            target_min: self.target_min,
        })
    }
}
