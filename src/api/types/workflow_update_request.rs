pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowUpdateRequest {
    /// New display name value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// New description value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether to make the workflow available in the chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    /// New tags. Replaces the existing tag list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WorkflowUpdateRequest {
    pub fn builder() -> WorkflowUpdateRequestBuilder {
        <WorkflowUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUpdateRequestBuilder {
    display_name: Option<String>,
    description: Option<String>,
    available_in_chat_assistant: Option<bool>,
    tags: Option<Vec<String>>,
}

impl WorkflowUpdateRequestBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn available_in_chat_assistant(mut self, value: bool) -> Self {
        self.available_in_chat_assistant = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUpdateRequest`].
    pub fn build(self) -> Result<WorkflowUpdateRequest, BuildError> {
        Ok(WorkflowUpdateRequest {
            display_name: self.display_name,
            description: self.description,
            available_in_chat_assistant: self.available_in_chat_assistant,
            tags: self.tags,
        })
    }
}
