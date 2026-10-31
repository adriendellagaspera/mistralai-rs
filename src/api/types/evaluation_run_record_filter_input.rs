pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunRecordFilterInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_filters: Option<GoalStatusFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_filters: Option<ContentSearchFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_filters: Option<ContentSearchFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_filters: Option<ScoreFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<EvaluationRunRecordFilterInputSort>,
}

impl EvaluationRunRecordFilterInput {
    pub fn builder() -> EvaluationRunRecordFilterInputBuilder {
        <EvaluationRunRecordFilterInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunRecordFilterInputBuilder {
    goal_filters: Option<GoalStatusFilterGroup>,
    input_filters: Option<ContentSearchFilterGroup>,
    output_filters: Option<ContentSearchFilterGroup>,
    score_filters: Option<ScoreFilterGroup>,
    sort: Option<EvaluationRunRecordFilterInputSort>,
}

impl EvaluationRunRecordFilterInputBuilder {
    pub fn goal_filters(mut self, value: GoalStatusFilterGroup) -> Self {
        self.goal_filters = Some(value);
        self
    }

    pub fn input_filters(mut self, value: ContentSearchFilterGroup) -> Self {
        self.input_filters = Some(value);
        self
    }

    pub fn output_filters(mut self, value: ContentSearchFilterGroup) -> Self {
        self.output_filters = Some(value);
        self
    }

    pub fn score_filters(mut self, value: ScoreFilterGroup) -> Self {
        self.score_filters = Some(value);
        self
    }

    pub fn sort(mut self, value: EvaluationRunRecordFilterInputSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunRecordFilterInput`].
    pub fn build(self) -> Result<EvaluationRunRecordFilterInput, BuildError> {
        Ok(EvaluationRunRecordFilterInput {
            goal_filters: self.goal_filters,
            input_filters: self.input_filters,
            output_filters: self.output_filters,
            score_filters: self.score_filters,
            sort: self.sort,
        })
    }
}
