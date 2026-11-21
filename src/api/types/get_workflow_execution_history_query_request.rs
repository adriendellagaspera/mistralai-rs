pub use crate::prelude::*;

/// Query parameters for get_workflow_execution_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl GetWorkflowExecutionHistoryQueryRequest {
    pub fn builder() -> GetWorkflowExecutionHistoryQueryRequestBuilder {
        <GetWorkflowExecutionHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionHistoryQueryRequestBuilder {
    decode_payloads: Option<bool>,
}

impl GetWorkflowExecutionHistoryQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionHistoryQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowExecutionHistoryQueryRequest, BuildError> {
        Ok(GetWorkflowExecutionHistoryQueryRequest {
            decode_payloads: self.decode_payloads,
        })
    }
}
