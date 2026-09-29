pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchiveModelResponse {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ArchiveModelResponseObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl ArchiveModelResponse {
    pub fn builder() -> ArchiveModelResponseBuilder {
        <ArchiveModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchiveModelResponseBuilder {
    id: Option<String>,
    object: Option<ArchiveModelResponseObject>,
    archived: Option<bool>,
}

impl ArchiveModelResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ArchiveModelResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArchiveModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ArchiveModelResponseBuilder::id)
    pub fn build(self) -> Result<ArchiveModelResponse, BuildError> {
        Ok(ArchiveModelResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
            archived: self.archived,
        })
    }
}
