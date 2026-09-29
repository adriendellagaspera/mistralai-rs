pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteFileResponse {
    /// The deletion status.
    #[serde(default)]
    pub deleted: bool,
    /// The ID of the deleted file.
    #[serde(default)]
    pub id: String,
    /// The object type that was deleted
    #[serde(default)]
    pub object: String,
}

impl DeleteFileResponse {
    pub fn builder() -> DeleteFileResponseBuilder {
        <DeleteFileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteFileResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
    object: Option<String>,
}

impl DeleteFileResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteFileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteFileResponseBuilder::deleted)
    /// - [`id`](DeleteFileResponseBuilder::id)
    /// - [`object`](DeleteFileResponseBuilder::object)
    pub fn build(self) -> Result<DeleteFileResponse, BuildError> {
        Ok(DeleteFileResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
