pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowWithWorkerStatus {
    /// Unique identifier of the workflow
    #[serde(default)]
    pub id: String,
    /// Name of the workflow
    #[serde(default)]
    pub name: String,
    /// Display name of the workflow
    #[serde(default)]
    pub display_name: String,
    /// Type of the workflow
    pub r#type: WorkflowType,
    /// Description of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Customer ID of the workflow
    #[serde(default)]
    pub customer_id: String,
    /// Workspace ID of the workflow
    #[serde(default)]
    pub workspace_id: String,
    /// Reserved namespace for shared workflows (e.g., 'shared:my-shared-workflow')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespace: Option<String>,
    /// Whether the workflow is available in chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    /// Whether the workflow is technical (e.g. SDK-managed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_technical: Option<bool>,
    /// Whether the workflow is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Tags for filtering and discovery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether the workflow is active
    #[serde(default)]
    pub active: bool,
}

impl WorkflowWithWorkerStatus {
    pub fn builder() -> WorkflowWithWorkerStatusBuilder {
        <WorkflowWithWorkerStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowWithWorkerStatusBuilder {
    id: Option<String>,
    name: Option<String>,
    display_name: Option<String>,
    r#type: Option<WorkflowType>,
    description: Option<String>,
    customer_id: Option<String>,
    workspace_id: Option<String>,
    shared_namespace: Option<String>,
    available_in_chat_assistant: Option<bool>,
    is_technical: Option<bool>,
    archived: Option<bool>,
    tags: Option<Vec<String>>,
    active: Option<bool>,
}

impl WorkflowWithWorkerStatusBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: WorkflowType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn shared_namespace(mut self, value: impl Into<String>) -> Self {
        self.shared_namespace = Some(value.into());
        self
    }

    pub fn available_in_chat_assistant(mut self, value: bool) -> Self {
        self.available_in_chat_assistant = Some(value);
        self
    }

    pub fn is_technical(mut self, value: bool) -> Self {
        self.is_technical = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowWithWorkerStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkflowWithWorkerStatusBuilder::id)
    /// - [`name`](WorkflowWithWorkerStatusBuilder::name)
    /// - [`display_name`](WorkflowWithWorkerStatusBuilder::display_name)
    /// - [`r#type`](WorkflowWithWorkerStatusBuilder::r#type)
    /// - [`customer_id`](WorkflowWithWorkerStatusBuilder::customer_id)
    /// - [`workspace_id`](WorkflowWithWorkerStatusBuilder::workspace_id)
    /// - [`active`](WorkflowWithWorkerStatusBuilder::active)
    pub fn build(self) -> Result<WorkflowWithWorkerStatus, BuildError> {
        Ok(WorkflowWithWorkerStatus {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            shared_namespace: self.shared_namespace,
            available_in_chat_assistant: self.available_in_chat_assistant,
            is_technical: self.is_technical,
            archived: self.archived,
            tags: self.tags,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
        })
    }
}
