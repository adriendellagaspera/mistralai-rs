pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportDatasetFromExplorerRequest {
    #[serde(default)]
    pub completion_event_ids: Vec<String>,
}

impl ImportDatasetFromExplorerRequest {
    pub fn builder() -> ImportDatasetFromExplorerRequestBuilder {
        <ImportDatasetFromExplorerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDatasetFromExplorerRequestBuilder {
    completion_event_ids: Option<Vec<String>>,
}

impl ImportDatasetFromExplorerRequestBuilder {
    pub fn completion_event_ids(mut self, value: Vec<String>) -> Self {
        self.completion_event_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportDatasetFromExplorerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completion_event_ids`](ImportDatasetFromExplorerRequestBuilder::completion_event_ids)
    pub fn build(self) -> Result<ImportDatasetFromExplorerRequest, BuildError> {
        Ok(ImportDatasetFromExplorerRequest {
            completion_event_ids: self
                .completion_event_ids
                .ok_or_else(|| BuildError::missing_field("completion_event_ids"))?,
        })
    }
}
