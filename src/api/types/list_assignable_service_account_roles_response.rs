pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAssignableServiceAccountRolesResponse {
    #[serde(default)]
    pub items: Vec<AssignableServiceAccountRole>,
}

impl ListAssignableServiceAccountRolesResponse {
    pub fn builder() -> ListAssignableServiceAccountRolesResponseBuilder {
        <ListAssignableServiceAccountRolesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAssignableServiceAccountRolesResponseBuilder {
    items: Option<Vec<AssignableServiceAccountRole>>,
}

impl ListAssignableServiceAccountRolesResponseBuilder {
    pub fn items(mut self, value: Vec<AssignableServiceAccountRole>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAssignableServiceAccountRolesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ListAssignableServiceAccountRolesResponseBuilder::items)
    pub fn build(self) -> Result<ListAssignableServiceAccountRolesResponse, BuildError> {
        Ok(ListAssignableServiceAccountRolesResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
