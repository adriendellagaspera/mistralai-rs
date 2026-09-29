pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProcessingStatus {
    #[serde(default)]
    pub document_id: String,
    /// Processing status of the document.
    pub process_status: ProcessStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<String>,
}

impl ProcessingStatus {
    pub fn builder() -> ProcessingStatusBuilder {
        <ProcessingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcessingStatusBuilder {
    document_id: Option<String>,
    process_status: Option<ProcessStatus>,
    processing_status: Option<String>,
}

impl ProcessingStatusBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn process_status(mut self, value: ProcessStatus) -> Self {
        self.process_status = Some(value);
        self
    }

    pub fn processing_status(mut self, value: impl Into<String>) -> Self {
        self.processing_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProcessingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](ProcessingStatusBuilder::document_id)
    /// - [`process_status`](ProcessingStatusBuilder::process_status)
    pub fn build(self) -> Result<ProcessingStatus, BuildError> {
        Ok(ProcessingStatus {
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            process_status: self
                .process_status
                .ok_or_else(|| BuildError::missing_field("process_status"))?,
            processing_status: self.processing_status,
        })
    }
}
