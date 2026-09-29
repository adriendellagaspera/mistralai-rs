pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionLogSearchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub results: Vec<ExecutionLogRecord>,
}

impl ExecutionLogSearchResponse {
    pub fn builder() -> ExecutionLogSearchResponseBuilder {
        <ExecutionLogSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionLogSearchResponseBuilder {
    next_cursor: Option<String>,
    results: Option<Vec<ExecutionLogRecord>>,
}

impl ExecutionLogSearchResponseBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ExecutionLogRecord>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionLogSearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ExecutionLogSearchResponseBuilder::results)
    pub fn build(self) -> Result<ExecutionLogSearchResponse, BuildError> {
        Ok(ExecutionLogSearchResponse {
            next_cursor: self.next_cursor,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
