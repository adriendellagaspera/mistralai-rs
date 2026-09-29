pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowRegistration {
    /// Unique identifier of the workflow registration
    #[serde(default)]
    pub id: String,
    /// Deprecated. Use deployment_name instead. Will be removed in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// Deprecated. Use deployment_name instead. Will be removed in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queue: Option<String>,
    #[serde(default)]
    pub definition: WorkflowCodeDefinition,
    /// Workflow ID of the workflow
    #[serde(default)]
    pub workflow_id: String,
    /// Workflow of the workflow registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// Name of the deployment this registration belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Whether the workflow is compatible with chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_with_chat_assistant: Option<bool>,
}

impl WorkflowRegistration {
    pub fn builder() -> WorkflowRegistrationBuilder {
        <WorkflowRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRegistrationBuilder {
    id: Option<String>,
    deployment_id: Option<String>,
    task_queue: Option<String>,
    definition: Option<WorkflowCodeDefinition>,
    workflow_id: Option<String>,
    workflow: Option<Workflow>,
    deployment_name: Option<String>,
    compatible_with_chat_assistant: Option<bool>,
}

impl WorkflowRegistrationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deployment_id(mut self, value: impl Into<String>) -> Self {
        self.deployment_id = Some(value.into());
        self
    }

    pub fn task_queue(mut self, value: impl Into<String>) -> Self {
        self.task_queue = Some(value.into());
        self
    }

    pub fn definition(mut self, value: WorkflowCodeDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn compatible_with_chat_assistant(mut self, value: bool) -> Self {
        self.compatible_with_chat_assistant = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkflowRegistrationBuilder::id)
    /// - [`definition`](WorkflowRegistrationBuilder::definition)
    /// - [`workflow_id`](WorkflowRegistrationBuilder::workflow_id)
    pub fn build(self) -> Result<WorkflowRegistration, BuildError> {
        Ok(WorkflowRegistration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deployment_id: self.deployment_id,
            task_queue: self.task_queue,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            workflow_id: self
                .workflow_id
                .ok_or_else(|| BuildError::missing_field("workflow_id"))?,
            workflow: self.workflow,
            deployment_name: self.deployment_name,
            compatible_with_chat_assistant: self.compatible_with_chat_assistant,
        })
    }
}
