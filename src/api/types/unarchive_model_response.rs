pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnarchiveModelResponse {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<UnarchiveModelResponseObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl UnarchiveModelResponse {
    pub fn builder() -> UnarchiveModelResponseBuilder {
        <UnarchiveModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnarchiveModelResponseBuilder {
    id: Option<String>,
    object: Option<UnarchiveModelResponseObject>,
    archived: Option<bool>,
}

impl UnarchiveModelResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: UnarchiveModelResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnarchiveModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnarchiveModelResponseBuilder::id)
    pub fn build(self) -> Result<UnarchiveModelResponse, BuildError> {
        Ok(UnarchiveModelResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object,
            archived: self.archived,
        })
    }
}
