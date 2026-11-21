pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowTagsAddRequest {
    /// Tags to add to the workflow. Existing tags are kept.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl WorkflowTagsAddRequest {
    pub fn builder() -> WorkflowTagsAddRequestBuilder {
        <WorkflowTagsAddRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowTagsAddRequestBuilder {
    tags: Option<Vec<String>>,
}

impl WorkflowTagsAddRequestBuilder {
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowTagsAddRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](WorkflowTagsAddRequestBuilder::tags)
    pub fn build(self) -> Result<WorkflowTagsAddRequest, BuildError> {
        Ok(WorkflowTagsAddRequest {
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}
