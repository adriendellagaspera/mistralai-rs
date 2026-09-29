pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnarchiveModelResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<UnarchiveModelResponseObject>,
}

impl UnarchiveModelResponse {
    pub fn builder() -> UnarchiveModelResponseBuilder {
        <UnarchiveModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnarchiveModelResponseBuilder {
    archived: Option<bool>,
    id: Option<String>,
    object: Option<UnarchiveModelResponseObject>,
}

impl UnarchiveModelResponseBuilder {
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: UnarchiveModelResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnarchiveModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnarchiveModelResponseBuilder::id)
    pub fn build(self) -> Result<UnarchiveModelResponse, BuildError> {
        Ok(UnarchiveModelResponse {
            archived: self.archived,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
        })
    }
}
