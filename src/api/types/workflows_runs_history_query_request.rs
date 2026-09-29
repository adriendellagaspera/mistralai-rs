pub use crate::prelude::*;

/// Query parameters for history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsRunsHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl WorkflowsRunsHistoryQueryRequest {
    pub fn builder() -> WorkflowsRunsHistoryQueryRequestBuilder {
        <WorkflowsRunsHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsRunsHistoryQueryRequestBuilder {
    decode_payloads: Option<bool>,
}

impl WorkflowsRunsHistoryQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsRunsHistoryQueryRequest`].
    pub fn build(self) -> Result<WorkflowsRunsHistoryQueryRequest, BuildError> {
        Ok(WorkflowsRunsHistoryQueryRequest {
            decode_payloads: self.decode_payloads,
        })
    }
}
