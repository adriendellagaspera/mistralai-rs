pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSpanEvaluations {
    #[serde(default)]
    pub span_evaluations: FeedResultGetSpanEvaluation,
}

impl GetSpanEvaluations {
    pub fn builder() -> GetSpanEvaluationsBuilder {
        <GetSpanEvaluationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanEvaluationsBuilder {
    span_evaluations: Option<FeedResultGetSpanEvaluation>,
}

impl GetSpanEvaluationsBuilder {
    pub fn span_evaluations(mut self, value: FeedResultGetSpanEvaluation) -> Self {
        self.span_evaluations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanEvaluations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`span_evaluations`](GetSpanEvaluationsBuilder::span_evaluations)
    pub fn build(self) -> Result<GetSpanEvaluations, BuildError> {
        Ok(GetSpanEvaluations {
            span_evaluations: self
                .span_evaluations
                .ok_or_else(|| BuildError::missing_field("span_evaluations"))?,
        })
    }
}
