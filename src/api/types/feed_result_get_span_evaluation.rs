pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetSpanEvaluation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetSpanEvaluation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FeedResultGetSpanEvaluation {
    pub fn builder() -> FeedResultGetSpanEvaluationBuilder {
        <FeedResultGetSpanEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetSpanEvaluationBuilder {
    results: Option<Vec<GetSpanEvaluation>>,
    next: Option<String>,
    cursor: Option<String>,
}

impl FeedResultGetSpanEvaluationBuilder {
    pub fn results(mut self, value: Vec<GetSpanEvaluation>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetSpanEvaluation`].
    pub fn build(self) -> Result<FeedResultGetSpanEvaluation, BuildError> {
        Ok(FeedResultGetSpanEvaluation {
            results: self.results,
            next: self.next,
            cursor: self.cursor,
        })
    }
}
