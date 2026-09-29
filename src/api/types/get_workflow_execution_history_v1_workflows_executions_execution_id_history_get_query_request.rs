pub use crate::prelude::*;

/// Query parameters for get_workflow_execution_history_v1_workflows_executions__execution_id__history_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest {
    pub fn builder(
    ) -> GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequestBuilder
    {
        <GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequestBuilder
{
    decode_payloads: Option<bool>,
}

impl GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest,
        BuildError,
    > {
        Ok(
            GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest {
                decode_payloads: self.decode_payloads,
            },
        )
    }
}
