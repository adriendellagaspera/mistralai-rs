pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionLogSearchResponse {
    #[serde(default)]
    pub results: Vec<ExecutionLogRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ExecutionLogSearchResponse {
    pub fn builder() -> ExecutionLogSearchResponseBuilder {
        <ExecutionLogSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionLogSearchResponseBuilder {
    results: Option<Vec<ExecutionLogRecord>>,
    next_cursor: Option<String>,
}

impl ExecutionLogSearchResponseBuilder {
    pub fn results(mut self, value: Vec<ExecutionLogRecord>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionLogSearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ExecutionLogSearchResponseBuilder::results)
    pub fn build(self) -> Result<ExecutionLogSearchResponse, BuildError> {
        Ok(ExecutionLogSearchResponse {
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
        })
    }
}
