pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchExecutionBody {
    /// List of execution IDs to process
    #[serde(default)]
    pub execution_ids: Vec<String>,
}

impl BatchExecutionBody {
    pub fn builder() -> BatchExecutionBodyBuilder {
        <BatchExecutionBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchExecutionBodyBuilder {
    execution_ids: Option<Vec<String>>,
}

impl BatchExecutionBodyBuilder {
    pub fn execution_ids(mut self, value: Vec<String>) -> Self {
        self.execution_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchExecutionBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_ids`](BatchExecutionBodyBuilder::execution_ids)
    pub fn build(self) -> Result<BatchExecutionBody, BuildError> {
        Ok(BatchExecutionBody {
            execution_ids: self
                .execution_ids
                .ok_or_else(|| BuildError::missing_field("execution_ids"))?,
        })
    }
}
