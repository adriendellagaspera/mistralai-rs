pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowListResponse {
    /// A list of workflows
    #[serde(default)]
    pub workflows: Vec<WorkflowBasicDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl WorkflowListResponse {
    pub fn builder() -> WorkflowListResponseBuilder {
        <WorkflowListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowListResponseBuilder {
    workflows: Option<Vec<WorkflowBasicDefinition>>,
    next_cursor: Option<String>,
}

impl WorkflowListResponseBuilder {
    pub fn workflows(mut self, value: Vec<WorkflowBasicDefinition>) -> Self {
        self.workflows = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflows`](WorkflowListResponseBuilder::workflows)
    pub fn build(self) -> Result<WorkflowListResponse, BuildError> {
        Ok(WorkflowListResponse {
            workflows: self
                .workflows
                .ok_or_else(|| BuildError::missing_field("workflows"))?,
            next_cursor: self.next_cursor,
        })
    }
}
