pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeploymentLogSearchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub results: Vec<DeploymentLogRecord>,
}

impl DeploymentLogSearchResponse {
    pub fn builder() -> DeploymentLogSearchResponseBuilder {
        <DeploymentLogSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentLogSearchResponseBuilder {
    next_cursor: Option<String>,
    results: Option<Vec<DeploymentLogRecord>>,
}

impl DeploymentLogSearchResponseBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<DeploymentLogRecord>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentLogSearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](DeploymentLogSearchResponseBuilder::results)
    pub fn build(self) -> Result<DeploymentLogSearchResponse, BuildError> {
        Ok(DeploymentLogSearchResponse {
            next_cursor: self.next_cursor,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
