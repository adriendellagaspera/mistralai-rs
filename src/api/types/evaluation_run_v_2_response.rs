pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunV2Response {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(default)]
    pub evaluators: Vec<EvaluatorV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_results: Option<HashMap<String, GoalResult>>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub num_generations: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization: Option<RunOptimizationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_evaluators: Option<Vec<EvaluatorV2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_info: Option<HashMap<String, Option<JsonValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_scores: Option<HashMap<String, EvaluationRunScoreV2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<HashMap<String, EvaluatorStatistics>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemV2>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl EvaluationRunV2Response {
    pub fn builder() -> EvaluationRunV2ResponseBuilder {
        <EvaluationRunV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunV2ResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    dataset_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    evaluation_id: Option<String>,
    evaluators: Option<Vec<EvaluatorV2>>,
    goal_results: Option<HashMap<String, GoalResult>>,
    id: Option<String>,
    metadata: Option<HashMap<String, JsonValue>>,
    name: Option<String>,
    num_generations: Option<i64>,
    optimization: Option<RunOptimizationRef>,
    passed: Option<bool>,
    run_evaluators: Option<Vec<EvaluatorV2>>,
    run_info: Option<HashMap<String, Option<JsonValue>>>,
    run_scores: Option<HashMap<String, EvaluationRunScoreV2>>,
    statistics: Option<HashMap<String, EvaluatorStatistics>>,
    status: Option<String>,
    system: Option<SystemV2>,
    tags: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl EvaluationRunV2ResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn evaluation_id(mut self, value: impl Into<String>) -> Self {
        self.evaluation_id = Some(value.into());
        self
    }

    pub fn evaluators(mut self, value: Vec<EvaluatorV2>) -> Self {
        self.evaluators = Some(value);
        self
    }

    pub fn goal_results(mut self, value: HashMap<String, GoalResult>) -> Self {
        self.goal_results = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn num_generations(mut self, value: i64) -> Self {
        self.num_generations = Some(value);
        self
    }

    pub fn optimization(mut self, value: RunOptimizationRef) -> Self {
        self.optimization = Some(value);
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn run_evaluators(mut self, value: Vec<EvaluatorV2>) -> Self {
        self.run_evaluators = Some(value);
        self
    }

    pub fn run_info(mut self, value: HashMap<String, Option<JsonValue>>) -> Self {
        self.run_info = Some(value);
        self
    }

    pub fn run_scores(mut self, value: HashMap<String, EvaluationRunScoreV2>) -> Self {
        self.run_scores = Some(value);
        self
    }

    pub fn statistics(mut self, value: HashMap<String, EvaluatorStatistics>) -> Self {
        self.statistics = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn system(mut self, value: SystemV2) -> Self {
        self.system = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationRunV2ResponseBuilder::created_at)
    /// - [`creator_id`](EvaluationRunV2ResponseBuilder::creator_id)
    /// - [`evaluation_id`](EvaluationRunV2ResponseBuilder::evaluation_id)
    /// - [`evaluators`](EvaluationRunV2ResponseBuilder::evaluators)
    /// - [`id`](EvaluationRunV2ResponseBuilder::id)
    /// - [`metadata`](EvaluationRunV2ResponseBuilder::metadata)
    /// - [`name`](EvaluationRunV2ResponseBuilder::name)
    /// - [`num_generations`](EvaluationRunV2ResponseBuilder::num_generations)
    /// - [`tags`](EvaluationRunV2ResponseBuilder::tags)
    /// - [`updated_at`](EvaluationRunV2ResponseBuilder::updated_at)
    pub fn build(self) -> Result<EvaluationRunV2Response, BuildError> {
        Ok(EvaluationRunV2Response {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            dataset_id: self.dataset_id,
            deleted_at: self.deleted_at,
            description: self.description,
            evaluation_id: self
                .evaluation_id
                .ok_or_else(|| BuildError::missing_field("evaluation_id"))?,
            evaluators: self
                .evaluators
                .ok_or_else(|| BuildError::missing_field("evaluators"))?,
            goal_results: self.goal_results,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            num_generations: self
                .num_generations
                .ok_or_else(|| BuildError::missing_field("num_generations"))?,
            optimization: self.optimization,
            passed: self.passed,
            run_evaluators: self.run_evaluators,
            run_info: self.run_info,
            run_scores: self.run_scores,
            statistics: self.statistics,
            status: self.status,
            system: self.system,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
