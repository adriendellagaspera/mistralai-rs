pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowMetadata {
    /// Namespace for shared workflows, None if user-owned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespace: Option<String>,
}

impl WorkflowMetadata {
    pub fn builder() -> WorkflowMetadataBuilder {
        <WorkflowMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowMetadataBuilder {
    shared_namespace: Option<String>,
}

impl WorkflowMetadataBuilder {
    pub fn shared_namespace(mut self, value: impl Into<String>) -> Self {
        self.shared_namespace = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowMetadata`].
    pub fn build(self) -> Result<WorkflowMetadata, BuildError> {
        Ok(WorkflowMetadata {
            shared_namespace: self.shared_namespace,
        })
    }
}
