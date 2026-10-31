pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResultEvaluationRunV2Response {
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EvaluationRunV2Response>>,
}

impl PaginatedResultEvaluationRunV2Response {
    pub fn builder() -> PaginatedResultEvaluationRunV2ResponseBuilder {
        <PaginatedResultEvaluationRunV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultEvaluationRunV2ResponseBuilder {
    count: Option<i64>,
    next: Option<String>,
    previous: Option<String>,
    results: Option<Vec<EvaluationRunV2Response>>,
}

impl PaginatedResultEvaluationRunV2ResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn previous(mut self, value: impl Into<String>) -> Self {
        self.previous = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<EvaluationRunV2Response>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultEvaluationRunV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PaginatedResultEvaluationRunV2ResponseBuilder::count)
    pub fn build(self) -> Result<PaginatedResultEvaluationRunV2Response, BuildError> {
        Ok(PaginatedResultEvaluationRunV2Response {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            next: self.next,
            previous: self.previous,
            results: self.results,
        })
    }
}
