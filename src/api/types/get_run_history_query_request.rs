pub use crate::prelude::*;

/// Query parameters for get_run_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRunHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_payloads: Option<bool>,
}

impl GetRunHistoryQueryRequest {
    pub fn builder() -> GetRunHistoryQueryRequestBuilder {
        <GetRunHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRunHistoryQueryRequestBuilder {
    decode_payloads: Option<bool>,
}

impl GetRunHistoryQueryRequestBuilder {
    pub fn decode_payloads(mut self, value: bool) -> Self {
        self.decode_payloads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRunHistoryQueryRequest`].
    pub fn build(self) -> Result<GetRunHistoryQueryRequest, BuildError> {
        Ok(GetRunHistoryQueryRequest {
            decode_payloads: self.decode_payloads,
        })
    }
}
