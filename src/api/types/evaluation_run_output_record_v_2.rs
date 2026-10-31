pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunOutputRecordV2 {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<EvaluationRunGenerationV2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_results: Option<HashMap<String, GoalResult>>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_info: Option<HashMap<String, Option<JsonValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<HashMap<String, EvaluatorStatistics>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl EvaluationRunOutputRecordV2 {
    pub fn builder() -> EvaluationRunOutputRecordV2Builder {
        <EvaluationRunOutputRecordV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunOutputRecordV2Builder {
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    generations: Option<Vec<EvaluationRunGenerationV2>>,
    goal_results: Option<HashMap<String, GoalResult>>,
    id: Option<String>,
    metadata: Option<HashMap<String, JsonValue>>,
    record_info: Option<HashMap<String, Option<JsonValue>>>,
    statistics: Option<HashMap<String, EvaluatorStatistics>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl EvaluationRunOutputRecordV2Builder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn generations(mut self, value: Vec<EvaluationRunGenerationV2>) -> Self {
        self.generations = Some(value);
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

    pub fn record_info(mut self, value: HashMap<String, Option<JsonValue>>) -> Self {
        self.record_info = Some(value);
        self
    }

    pub fn statistics(mut self, value: HashMap<String, EvaluatorStatistics>) -> Self {
        self.statistics = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunOutputRecordV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationRunOutputRecordV2Builder::created_at)
    /// - [`id`](EvaluationRunOutputRecordV2Builder::id)
    /// - [`metadata`](EvaluationRunOutputRecordV2Builder::metadata)
    /// - [`updated_at`](EvaluationRunOutputRecordV2Builder::updated_at)
    pub fn build(self) -> Result<EvaluationRunOutputRecordV2, BuildError> {
        Ok(EvaluationRunOutputRecordV2 {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            generations: self.generations,
            goal_results: self.goal_results,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            record_info: self.record_info,
            statistics: self.statistics,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
