pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchExecutionResponse {
    /// Mapping of execution_id to result with status and optional error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<HashMap<String, BatchExecutionResult>>,
}

impl BatchExecutionResponse {
    pub fn builder() -> BatchExecutionResponseBuilder {
        <BatchExecutionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchExecutionResponseBuilder {
    results: Option<HashMap<String, BatchExecutionResult>>,
}

impl BatchExecutionResponseBuilder {
    pub fn results(mut self, value: HashMap<String, BatchExecutionResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchExecutionResponse`].
    pub fn build(self) -> Result<BatchExecutionResponse, BuildError> {
        Ok(BatchExecutionResponse {
            results: self.results,
        })
    }
}
