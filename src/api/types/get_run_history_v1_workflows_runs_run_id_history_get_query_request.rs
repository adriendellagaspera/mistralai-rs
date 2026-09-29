pub use crate::prelude::*;

/// Query parameters for get_run_history_v1_workflows_runs__run_id__history_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequest {
    pub fn builder() -> GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequestBuilder {
        <GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequestBuilder {
    decode_payloads: Option<bool>,
}

impl GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequest, BuildError> {
        Ok(GetRunHistoryV1WorkflowsRunsRunIdHistoryGetQueryRequest {
            decode_payloads: self.decode_payloads,
        })
    }
}
