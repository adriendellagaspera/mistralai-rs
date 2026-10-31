pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunStatisticsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_results: Option<HashMap<String, GoalResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub statistics: HashMap<String, EvaluatorStatistics>,
}

impl RunStatisticsResponse {
    pub fn builder() -> RunStatisticsResponseBuilder {
        <RunStatisticsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunStatisticsResponseBuilder {
    goal_results: Option<HashMap<String, GoalResult>>,
    passed: Option<bool>,
    run_id: Option<String>,
    statistics: Option<HashMap<String, EvaluatorStatistics>>,
}

impl RunStatisticsResponseBuilder {
    pub fn goal_results(mut self, value: HashMap<String, GoalResult>) -> Self {
        self.goal_results = Some(value);
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn statistics(mut self, value: HashMap<String, EvaluatorStatistics>) -> Self {
        self.statistics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunStatisticsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](RunStatisticsResponseBuilder::run_id)
    /// - [`statistics`](RunStatisticsResponseBuilder::statistics)
    pub fn build(self) -> Result<RunStatisticsResponse, BuildError> {
        Ok(RunStatisticsResponse {
            goal_results: self.goal_results,
            passed: self.passed,
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
            statistics: self
                .statistics
                .ok_or_else(|| BuildError::missing_field("statistics"))?,
        })
    }
}
