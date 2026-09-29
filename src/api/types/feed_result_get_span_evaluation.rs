pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetSpanEvaluation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetSpanEvaluation>>,
}

impl FeedResultGetSpanEvaluation {
    pub fn builder() -> FeedResultGetSpanEvaluationBuilder {
        <FeedResultGetSpanEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetSpanEvaluationBuilder {
    cursor: Option<String>,
    next: Option<String>,
    results: Option<Vec<GetSpanEvaluation>>,
}

impl FeedResultGetSpanEvaluationBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<GetSpanEvaluation>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetSpanEvaluation`].
    pub fn build(self) -> Result<FeedResultGetSpanEvaluation, BuildError> {
        Ok(FeedResultGetSpanEvaluation {
            cursor: self.cursor,
            next: self.next,
            results: self.results,
        })
    }
}
