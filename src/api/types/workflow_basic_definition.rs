pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBasicDefinition {
    /// Whether the workflow is archived
    #[serde(default)]
    pub archived: bool,
    /// A description of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the workflow
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    /// Workflow metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WorkflowMetadata>,
    /// The name of the workflow
    #[serde(default)]
    pub name: String,
    /// Workflow tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WorkflowBasicDefinition {
    pub fn builder() -> WorkflowBasicDefinitionBuilder {
        <WorkflowBasicDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBasicDefinitionBuilder {
    archived: Option<bool>,
    description: Option<String>,
    display_name: Option<String>,
    id: Option<String>,
    metadata: Option<WorkflowMetadata>,
    name: Option<String>,
    tags: Option<Vec<String>>,
}

impl WorkflowBasicDefinitionBuilder {
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
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

    pub fn metadata(mut self, value: WorkflowMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBasicDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`archived`](WorkflowBasicDefinitionBuilder::archived)
    /// - [`display_name`](WorkflowBasicDefinitionBuilder::display_name)
    /// - [`id`](WorkflowBasicDefinitionBuilder::id)
    /// - [`name`](WorkflowBasicDefinitionBuilder::name)
    pub fn build(self) -> Result<WorkflowBasicDefinition, BuildError> {
        Ok(WorkflowBasicDefinition {
            archived: self
                .archived
                .ok_or_else(|| BuildError::missing_field("archived"))?,
            description: self.description,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tags: self.tags,
        })
    }
}
