pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteModelResponse {
    /// The deletion status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The ID of the deleted model.
    #[serde(default)]
    pub id: String,
    /// The object type that was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

impl DeleteModelResponse {
    pub fn builder() -> DeleteModelResponseBuilder {
        <DeleteModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteModelResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
    object: Option<String>,
}

impl DeleteModelResponseBuilder {
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

    /// Consumes the builder and constructs a [`DeleteModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteModelResponseBuilder::id)
    pub fn build(self) -> Result<DeleteModelResponse, BuildError> {
        Ok(DeleteModelResponse {
            deleted: self.deleted,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
        })
    }
}
