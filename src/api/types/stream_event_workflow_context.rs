pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamEventWorkflowContext {
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub workflow_name: String,
    #[serde(default)]
    pub workflow_exec_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_workflow_exec_id: Option<String>,
}

impl StreamEventWorkflowContext {
    pub fn builder() -> StreamEventWorkflowContextBuilder {
        <StreamEventWorkflowContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventWorkflowContextBuilder {
    namespace: Option<String>,
    workflow_name: Option<String>,
    workflow_exec_id: Option<String>,
    parent_workflow_exec_id: Option<String>,
    root_workflow_exec_id: Option<String>,
}

impl StreamEventWorkflowContextBuilder {
    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_exec_id = Some(value.into());
        self
    }

    pub fn parent_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.parent_workflow_exec_id = Some(value.into());
        self
    }

    pub fn root_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.root_workflow_exec_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamEventWorkflowContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`namespace`](StreamEventWorkflowContextBuilder::namespace)
    /// - [`workflow_name`](StreamEventWorkflowContextBuilder::workflow_name)
    /// - [`workflow_exec_id`](StreamEventWorkflowContextBuilder::workflow_exec_id)
    pub fn build(self) -> Result<StreamEventWorkflowContext, BuildError> {
        Ok(StreamEventWorkflowContext {
            namespace: self
                .namespace
                .ok_or_else(|| BuildError::missing_field("namespace"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            workflow_exec_id: self
                .workflow_exec_id
                .ok_or_else(|| BuildError::missing_field("workflow_exec_id"))?,
            parent_workflow_exec_id: self.parent_workflow_exec_id,
            root_workflow_exec_id: self.root_workflow_exec_id,
        })
    }
}
