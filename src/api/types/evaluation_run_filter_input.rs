pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunFilterInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_filters: Option<GoalStatusFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<RunOwnershipFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_filters: Option<ScoreFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagFilter>,
}

impl EvaluationRunFilterInput {
    pub fn builder() -> EvaluationRunFilterInputBuilder {
        <EvaluationRunFilterInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunFilterInputBuilder {
    creator_ids: Option<Vec<String>>,
    evaluation_ids: Option<Vec<String>>,
    goal_filters: Option<GoalStatusFilterGroup>,
    metadata: Option<MetadataFilterGroup>,
    ownership: Option<RunOwnershipFilter>,
    project_ids: Option<Vec<String>>,
    run_ids: Option<Vec<String>>,
    score_filters: Option<ScoreFilterGroup>,
    system: Option<SystemFilter>,
    tags: Option<TagFilter>,
}

impl EvaluationRunFilterInputBuilder {
    pub fn creator_ids(mut self, value: Vec<String>) -> Self {
        self.creator_ids = Some(value);
        self
    }

    pub fn evaluation_ids(mut self, value: Vec<String>) -> Self {
        self.evaluation_ids = Some(value);
        self
    }

    pub fn goal_filters(mut self, value: GoalStatusFilterGroup) -> Self {
        self.goal_filters = Some(value);
        self
    }

    pub fn metadata(mut self, value: MetadataFilterGroup) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn ownership(mut self, value: RunOwnershipFilter) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    pub fn run_ids(mut self, value: Vec<String>) -> Self {
        self.run_ids = Some(value);
        self
    }

    pub fn score_filters(mut self, value: ScoreFilterGroup) -> Self {
        self.score_filters = Some(value);
        self
    }

    pub fn system(mut self, value: SystemFilter) -> Self {
        self.system = Some(value);
        self
    }

    pub fn tags(mut self, value: TagFilter) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunFilterInput`].
    pub fn build(self) -> Result<EvaluationRunFilterInput, BuildError> {
        Ok(EvaluationRunFilterInput {
            creator_ids: self.creator_ids,
            evaluation_ids: self.evaluation_ids,
            goal_filters: self.goal_filters,
            metadata: self.metadata,
            ownership: self.ownership,
            project_ids: self.project_ids,
            run_ids: self.run_ids,
            score_filters: self.score_filters,
            system: self.system,
            tags: self.tags,
        })
    }
}
