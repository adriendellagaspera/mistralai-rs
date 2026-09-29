pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryWorkflowResponse {
    #[serde(default)]
    pub query_name: String,
    pub result: serde_json::Value,
}

impl QueryWorkflowResponse {
    pub fn builder() -> QueryWorkflowResponseBuilder {
        <QueryWorkflowResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryWorkflowResponseBuilder {
    query_name: Option<String>,
    result: Option<serde_json::Value>,
}

impl QueryWorkflowResponseBuilder {
    pub fn query_name(mut self, value: impl Into<String>) -> Self {
        self.query_name = Some(value.into());
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryWorkflowResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query_name`](QueryWorkflowResponseBuilder::query_name)
    /// - [`result`](QueryWorkflowResponseBuilder::result)
    pub fn build(self) -> Result<QueryWorkflowResponse, BuildError> {
        Ok(QueryWorkflowResponse {
            query_name: self
                .query_name
                .ok_or_else(|| BuildError::missing_field("query_name"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
