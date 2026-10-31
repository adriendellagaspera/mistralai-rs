pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunGenerationV2 {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_results: Option<HashMap<String, GoalResult>>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores: Option<HashMap<String, Vec<EvaluationRunScoreV2>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<HashMap<String, EvaluatorStatistics>>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl EvaluationRunGenerationV2 {
    pub fn builder() -> EvaluationRunGenerationV2Builder {
        <EvaluationRunGenerationV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunGenerationV2Builder {
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    goal_results: Option<HashMap<String, GoalResult>>,
    id: Option<String>,
    metadata: Option<HashMap<String, JsonValue>>,
    output: Option<JsonValue>,
    scores: Option<HashMap<String, Vec<EvaluationRunScoreV2>>>,
    statistics: Option<HashMap<String, EvaluatorStatistics>>,
    status: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl EvaluationRunGenerationV2Builder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
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

    pub fn output(mut self, value: JsonValue) -> Self {
        self.output = Some(value);
        self
    }

    pub fn scores(mut self, value: HashMap<String, Vec<EvaluationRunScoreV2>>) -> Self {
        self.scores = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunGenerationV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationRunGenerationV2Builder::created_at)
    /// - [`id`](EvaluationRunGenerationV2Builder::id)
    /// - [`metadata`](EvaluationRunGenerationV2Builder::metadata)
    /// - [`status`](EvaluationRunGenerationV2Builder::status)
    /// - [`updated_at`](EvaluationRunGenerationV2Builder::updated_at)
    pub fn build(self) -> Result<EvaluationRunGenerationV2, BuildError> {
        Ok(EvaluationRunGenerationV2 {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            error: self.error,
            goal_results: self.goal_results,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            output: self.output,
            scores: self.scores,
            statistics: self.statistics,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
