pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OptimizationDetail {
    #[serde(default)]
    pub algorithm: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<OptimizationReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_spec: Option<HashMap<String, Option<JsonValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<HashMap<String, Option<JsonValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<OptimizationReference>,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<String>,
}

impl OptimizationDetail {
    pub fn builder() -> OptimizationDetailBuilder {
        <OptimizationDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptimizationDetailBuilder {
    algorithm: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    evaluation: Option<OptimizationReference>,
    evaluation_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    objective_spec: Option<HashMap<String, Option<JsonValue>>>,
    outcome: Option<HashMap<String, Option<JsonValue>>>,
    project: Option<OptimizationReference>,
    project_id: Option<String>,
    source: Option<String>,
    status: Option<String>,
    steer: Option<String>,
    tags: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
    workflow_execution_id: Option<String>,
    workflow_run_id: Option<String>,
}

impl OptimizationDetailBuilder {
    pub fn algorithm(mut self, value: impl Into<String>) -> Self {
        self.algorithm = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
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

    pub fn evaluation(mut self, value: OptimizationReference) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn evaluation_id(mut self, value: impl Into<String>) -> Self {
        self.evaluation_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn objective_spec(mut self, value: HashMap<String, Option<JsonValue>>) -> Self {
        self.objective_spec = Some(value);
        self
    }

    pub fn outcome(mut self, value: HashMap<String, Option<JsonValue>>) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn project(mut self, value: OptimizationReference) -> Self {
        self.project = Some(value);
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn steer(mut self, value: impl Into<String>) -> Self {
        self.steer = Some(value.into());
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

    pub fn workflow_execution_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_execution_id = Some(value.into());
        self
    }

    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OptimizationDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`algorithm`](OptimizationDetailBuilder::algorithm)
    /// - [`created_at`](OptimizationDetailBuilder::created_at)
    /// - [`creator_id`](OptimizationDetailBuilder::creator_id)
    /// - [`id`](OptimizationDetailBuilder::id)
    /// - [`name`](OptimizationDetailBuilder::name)
    /// - [`project_id`](OptimizationDetailBuilder::project_id)
    /// - [`source`](OptimizationDetailBuilder::source)
    /// - [`status`](OptimizationDetailBuilder::status)
    /// - [`updated_at`](OptimizationDetailBuilder::updated_at)
    pub fn build(self) -> Result<OptimizationDetail, BuildError> {
        Ok(OptimizationDetail {
            algorithm: self
                .algorithm
                .ok_or_else(|| BuildError::missing_field("algorithm"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            deleted_at: self.deleted_at,
            description: self.description,
            evaluation: self.evaluation,
            evaluation_id: self.evaluation_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            objective_spec: self.objective_spec,
            outcome: self.outcome,
            project: self.project,
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            steer: self.steer,
            tags: self.tags,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workflow_execution_id: self.workflow_execution_id,
            workflow_run_id: self.workflow_run_id,
        })
    }
}
