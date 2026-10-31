pub use crate::prelude::*;

/// Paginated list of managed indexes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListManagedIndexesResponse {
    #[serde(default)]
    pub data: Vec<ManagedIndexResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListManagedIndexesResponse {
    pub fn builder() -> ListManagedIndexesResponseBuilder {
        <ListManagedIndexesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListManagedIndexesResponseBuilder {
    data: Option<Vec<ManagedIndexResponse>>,
    next_page_token: Option<String>,
}

impl ListManagedIndexesResponseBuilder {
    pub fn data(mut self, value: Vec<ManagedIndexResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListManagedIndexesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListManagedIndexesResponseBuilder::data)
    pub fn build(self) -> Result<ListManagedIndexesResponse, BuildError> {
        Ok(ListManagedIndexesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
        })
    }
}
