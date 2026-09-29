pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFilesResponse {
    #[serde(default)]
    pub data: Vec<FileSchema>,
    #[serde(default)]
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl ListFilesResponse {
    pub fn builder() -> ListFilesResponseBuilder {
        <ListFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesResponseBuilder {
    data: Option<Vec<FileSchema>>,
    object: Option<String>,
    total: Option<i64>,
}

impl ListFilesResponseBuilder {
    pub fn data(mut self, value: Vec<FileSchema>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFilesResponseBuilder::data)
    /// - [`object`](ListFilesResponseBuilder::object)
    pub fn build(self) -> Result<ListFilesResponse, BuildError> {
        Ok(ListFilesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            total: self.total,
        })
    }
}
