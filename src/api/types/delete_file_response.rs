pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteFileResponse {
    /// The ID of the deleted file.
    #[serde(default)]
    pub id: String,
    /// The object type that was deleted
    #[serde(default)]
    pub object: String,
    /// The deletion status.
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteFileResponse {
    pub fn builder() -> DeleteFileResponseBuilder {
        <DeleteFileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteFileResponseBuilder {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
}

impl DeleteFileResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteFileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteFileResponseBuilder::id)
    /// - [`object`](DeleteFileResponseBuilder::object)
    /// - [`deleted`](DeleteFileResponseBuilder::deleted)
    pub fn build(self) -> Result<DeleteFileResponse, BuildError> {
        Ok(DeleteFileResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
