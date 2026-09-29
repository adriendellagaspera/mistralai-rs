pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowRegistration {
    /// Whether the workflow is compatible with chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_with_chat_assistant: Option<bool>,
    #[serde(default)]
    pub definition: WorkflowCodeDefinition,
    /// Deprecated. Use deployment_name instead. Will be removed in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// Name of the deployment this registration belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Unique identifier of the workflow registration
    #[serde(default)]
    pub id: String,
    /// Deprecated. Use deployment_name instead. Will be removed in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queue: Option<String>,
    /// Workflow of the workflow registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// Workflow ID of the workflow
    #[serde(default)]
    pub workflow_id: String,
}

impl WorkflowRegistration {
    pub fn builder() -> WorkflowRegistrationBuilder {
        <WorkflowRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRegistrationBuilder {
    compatible_with_chat_assistant: Option<bool>,
    definition: Option<WorkflowCodeDefinition>,
    deployment_id: Option<String>,
    deployment_name: Option<String>,
    id: Option<String>,
    task_queue: Option<String>,
    workflow: Option<Workflow>,
    workflow_id: Option<String>,
}

impl WorkflowRegistrationBuilder {
    pub fn compatible_with_chat_assistant(mut self, value: bool) -> Self {
        self.compatible_with_chat_assistant = Some(value);
        self
    }

    pub fn definition(mut self, value: WorkflowCodeDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn deployment_id(mut self, value: impl Into<String>) -> Self {
        self.deployment_id = Some(value.into());
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn task_queue(mut self, value: impl Into<String>) -> Self {
        self.task_queue = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](WorkflowRegistrationBuilder::definition)
    /// - [`id`](WorkflowRegistrationBuilder::id)
    /// - [`workflow_id`](WorkflowRegistrationBuilder::workflow_id)
    pub fn build(self) -> Result<WorkflowRegistration, BuildError> {
        Ok(WorkflowRegistration {
            compatible_with_chat_assistant: self.compatible_with_chat_assistant,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            deployment_id: self.deployment_id,
            deployment_name: self.deployment_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            task_queue: self.task_queue,
            workflow: self.workflow,
            workflow_id: self
                .workflow_id
                .ok_or_else(|| BuildError::missing_field("workflow_id"))?,
        })
    }
}
