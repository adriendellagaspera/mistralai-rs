pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Workflow {
    /// Whether the workflow is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Whether the workflow is available in chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    /// Customer ID of the workflow
    #[serde(default)]
    pub customer_id: String,
    /// Description of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Display name of the workflow
    #[serde(default)]
    pub display_name: String,
    /// Unique identifier of the workflow
    #[serde(default)]
    pub id: String,
    /// Whether the workflow is technical (e.g. SDK-managed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_technical: Option<bool>,
    /// Name of the workflow
    #[serde(default)]
    pub name: String,
    /// Reserved namespace for shared workflows (e.g., 'shared:my-shared-workflow')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_namespace: Option<String>,
    /// Tags for filtering and discovery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Type of the workflow
    pub r#type: WorkflowType,
    /// Workspace ID of the workflow
    #[serde(default)]
    pub workspace_id: String,
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder {
        <WorkflowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBuilder {
    archived: Option<bool>,
    available_in_chat_assistant: Option<bool>,
    customer_id: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    id: Option<String>,
    is_technical: Option<bool>,
    name: Option<String>,
    shared_namespace: Option<String>,
    tags: Option<Vec<String>>,
    r#type: Option<WorkflowType>,
    workspace_id: Option<String>,
}

impl WorkflowBuilder {
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn available_in_chat_assistant(mut self, value: bool) -> Self {
        self.available_in_chat_assistant = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_technical(mut self, value: bool) -> Self {
        self.is_technical = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn shared_namespace(mut self, value: impl Into<String>) -> Self {
        self.shared_namespace = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn r#type(mut self, value: WorkflowType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Workflow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](WorkflowBuilder::customer_id)
    /// - [`display_name`](WorkflowBuilder::display_name)
    /// - [`id`](WorkflowBuilder::id)
    /// - [`name`](WorkflowBuilder::name)
    /// - [`r#type`](WorkflowBuilder::r#type)
    /// - [`workspace_id`](WorkflowBuilder::workspace_id)
    pub fn build(self) -> Result<Workflow, BuildError> {
        Ok(Workflow {
            archived: self.archived,
            available_in_chat_assistant: self.available_in_chat_assistant,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            description: self.description,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_technical: self.is_technical,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            shared_namespace: self.shared_namespace,
            tags: self.tags,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
