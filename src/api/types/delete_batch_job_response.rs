pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteBatchJobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<DeleteBatchJobResponseObject>,
}

impl DeleteBatchJobResponse {
    pub fn builder() -> DeleteBatchJobResponseBuilder {
        <DeleteBatchJobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteBatchJobResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
    object: Option<DeleteBatchJobResponseObject>,
}

impl DeleteBatchJobResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: DeleteBatchJobResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteBatchJobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteBatchJobResponseBuilder::id)
    pub fn build(self) -> Result<DeleteBatchJobResponse, BuildError> {
        Ok(DeleteBatchJobResponse {
            deleted: self.deleted,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
        })
    }
}
