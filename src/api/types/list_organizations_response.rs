pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrganizationsResponse {
    /// The organizations the authenticated user is a member of.
    #[serde(default)]
    pub organizations: Vec<UserOrganization>,
}

impl ListOrganizationsResponse {
    pub fn builder() -> ListOrganizationsResponseBuilder {
        <ListOrganizationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrganizationsResponseBuilder {
    organizations: Option<Vec<UserOrganization>>,
}

impl ListOrganizationsResponseBuilder {
    pub fn organizations(mut self, value: Vec<UserOrganization>) -> Self {
        self.organizations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrganizationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organizations`](ListOrganizationsResponseBuilder::organizations)
    pub fn build(self) -> Result<ListOrganizationsResponse, BuildError> {
        Ok(ListOrganizationsResponse {
            organizations: self
                .organizations
                .ok_or_else(|| BuildError::missing_field("organizations"))?,
        })
    }
}
