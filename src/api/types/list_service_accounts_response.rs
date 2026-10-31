pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListServiceAccountsResponse {
    #[serde(default)]
    pub items: Vec<ServiceAccount>,
}

impl ListServiceAccountsResponse {
    pub fn builder() -> ListServiceAccountsResponseBuilder {
        <ListServiceAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListServiceAccountsResponseBuilder {
    items: Option<Vec<ServiceAccount>>,
}

impl ListServiceAccountsResponseBuilder {
    pub fn items(mut self, value: Vec<ServiceAccount>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListServiceAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ListServiceAccountsResponseBuilder::items)
    pub fn build(self) -> Result<ListServiceAccountsResponse, BuildError> {
        Ok(ListServiceAccountsResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
