pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteModelResponse {
    /// The ID of the deleted model.
    #[serde(default)]
    pub id: String,
    /// The object type that was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The deletion status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl DeleteModelResponse {
    pub fn builder() -> DeleteModelResponseBuilder {
        <DeleteModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteModelResponseBuilder {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
}

impl DeleteModelResponseBuilder {
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

    /// Consumes the builder and constructs a [`DeleteModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteModelResponseBuilder::id)
    pub fn build(self) -> Result<DeleteModelResponse, BuildError> {
        Ok(DeleteModelResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
            deleted: self.deleted,
        })
    }
}
