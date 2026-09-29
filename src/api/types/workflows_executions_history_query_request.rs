pub use crate::prelude::*;

/// Query parameters for history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsExecutionsHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl WorkflowsExecutionsHistoryQueryRequest {
    pub fn builder() -> WorkflowsExecutionsHistoryQueryRequestBuilder {
        <WorkflowsExecutionsHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsExecutionsHistoryQueryRequestBuilder {
    decode_payloads: Option<bool>,
}

impl WorkflowsExecutionsHistoryQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsExecutionsHistoryQueryRequest`].
    pub fn build(self) -> Result<WorkflowsExecutionsHistoryQueryRequest, BuildError> {
        Ok(WorkflowsExecutionsHistoryQueryRequest {
            decode_payloads: self.decode_payloads,
        })
    }
}
