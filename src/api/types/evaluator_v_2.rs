pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluatorV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_goal: Option<GoalSpec>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<EvaluatorV2Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<GoalSpec>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub num_scores: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<EvaluatorV2StatisticsItem>>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub weight: Option<f64>,
}

impl EvaluatorV2 {
    pub fn builder() -> EvaluatorV2Builder {
        <EvaluatorV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluatorV2Builder {
    aggregate_goal: Option<GoalSpec>,
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    direction: Option<EvaluatorV2Direction>,
    goal: Option<GoalSpec>,
    id: Option<String>,
    max_value: Option<f64>,
    min_value: Option<f64>,
    name: Option<String>,
    num_scores: Option<i64>,
    statistics: Option<Vec<EvaluatorV2StatisticsItem>>,
    tags: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
    weight: Option<f64>,
}

impl EvaluatorV2Builder {
    pub fn aggregate_goal(mut self, value: GoalSpec) -> Self {
        self.aggregate_goal = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn direction(mut self, value: EvaluatorV2Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn goal(mut self, value: GoalSpec) -> Self {
        self.goal = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = Some(value);
        self
    }

    pub fn min_value(mut self, value: f64) -> Self {
        self.min_value = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn num_scores(mut self, value: i64) -> Self {
        self.num_scores = Some(value);
        self
    }

    pub fn statistics(mut self, value: Vec<EvaluatorV2StatisticsItem>) -> Self {
        self.statistics = Some(value);
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

    pub fn weight(mut self, value: f64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluatorV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluatorV2Builder::created_at)
    /// - [`id`](EvaluatorV2Builder::id)
    /// - [`name`](EvaluatorV2Builder::name)
    /// - [`num_scores`](EvaluatorV2Builder::num_scores)
    /// - [`tags`](EvaluatorV2Builder::tags)
    /// - [`updated_at`](EvaluatorV2Builder::updated_at)
    pub fn build(self) -> Result<EvaluatorV2, BuildError> {
        Ok(EvaluatorV2 {
            aggregate_goal: self.aggregate_goal,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            description: self.description,
            direction: self.direction,
            goal: self.goal,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_value: self.max_value,
            min_value: self.min_value,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            num_scores: self
                .num_scores
                .ok_or_else(|| BuildError::missing_field("num_scores"))?,
            statistics: self.statistics,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            weight: self.weight,
        })
    }
}
