pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteBatchJobResponse {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<DeleteBatchJobResponseObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl DeleteBatchJobResponse {
    pub fn builder() -> DeleteBatchJobResponseBuilder {
        <DeleteBatchJobResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteBatchJobResponseBuilder {
    id: Option<String>,
    object: Option<DeleteBatchJobResponseObject>,
    deleted: Option<bool>,
}

impl DeleteBatchJobResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: DeleteBatchJobResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteBatchJobResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteBatchJobResponseBuilder::id)
    pub fn build(self) -> Result<DeleteBatchJobResponse, BuildError> {
        Ok(DeleteBatchJobResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
            deleted: self.deleted,
        })
    }
}
