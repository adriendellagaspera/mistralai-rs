pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListServiceAccountRolesResponse {
    #[serde(default)]
    pub items: Vec<ServiceAccountRole>,
}

impl ListServiceAccountRolesResponse {
    pub fn builder() -> ListServiceAccountRolesResponseBuilder {
        <ListServiceAccountRolesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListServiceAccountRolesResponseBuilder {
    items: Option<Vec<ServiceAccountRole>>,
}

impl ListServiceAccountRolesResponseBuilder {
    pub fn items(mut self, value: Vec<ServiceAccountRole>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListServiceAccountRolesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ListServiceAccountRolesResponseBuilder::items)
    pub fn build(self) -> Result<ListServiceAccountRolesResponse, BuildError> {
        Ok(ListServiceAccountRolesResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
