pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<UserIdentityApiKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<UserIdentityOrganization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<UserIdentityWorkspace>,
}

impl UserIdentity {
    pub fn builder() -> UserIdentityBuilder {
        <UserIdentityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserIdentityBuilder {
    api_key: Option<UserIdentityApiKey>,
    email: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    organization: Option<UserIdentityOrganization>,
    workspace: Option<UserIdentityWorkspace>,
}

impl UserIdentityBuilder {
    pub fn api_key(mut self, value: UserIdentityApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn organization(mut self, value: UserIdentityOrganization) -> Self {
        self.organization = Some(value);
        self
    }

    pub fn workspace(mut self, value: UserIdentityWorkspace) -> Self {
        self.workspace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserIdentity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserIdentityBuilder::id)
    pub fn build(self) -> Result<UserIdentity, BuildError> {
        Ok(UserIdentity {
            api_key: self.api_key,
            email: self.email,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_name: self.last_name,
            organization: self.organization,
            workspace: self.workspace,
        })
    }
}
