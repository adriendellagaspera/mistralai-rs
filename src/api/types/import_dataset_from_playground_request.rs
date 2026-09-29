pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportDatasetFromPlaygroundRequest {
    #[serde(default)]
    pub conversation_ids: Vec<String>,
}

impl ImportDatasetFromPlaygroundRequest {
    pub fn builder() -> ImportDatasetFromPlaygroundRequestBuilder {
        <ImportDatasetFromPlaygroundRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDatasetFromPlaygroundRequestBuilder {
    conversation_ids: Option<Vec<String>>,
}

impl ImportDatasetFromPlaygroundRequestBuilder {
    pub fn conversation_ids(mut self, value: Vec<String>) -> Self {
        self.conversation_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportDatasetFromPlaygroundRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_ids`](ImportDatasetFromPlaygroundRequestBuilder::conversation_ids)
    pub fn build(self) -> Result<ImportDatasetFromPlaygroundRequest, BuildError> {
        Ok(ImportDatasetFromPlaygroundRequest {
            conversation_ids: self
                .conversation_ids
                .ok_or_else(|| BuildError::missing_field("conversation_ids"))?,
        })
    }
}
