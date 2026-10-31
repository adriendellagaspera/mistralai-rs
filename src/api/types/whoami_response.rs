pub use crate::prelude::*;

/// Principal-agnostic identity of the authenticated caller.
///
/// Supersedes the user-centric ``/v1/users/me`` and ``/api/users/v2/me`` by
/// resolving any principal (user or service account) through a single call. The
/// ``principal`` field is a discriminated union; the remaining fields are the
/// shared session context and apply to either principal kind.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhoamiResponse {
    /// The organization the caller's session is scoped to, when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<WhoamiOrganization>,
    /// The authenticated principal: either a user or a service account.
    pub principal: WhoamiResponsePrincipal,
    /// The workspace the caller's session is scoped to, when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<UserIdentityWorkspace>,
}

impl WhoamiResponse {
    pub fn builder() -> WhoamiResponseBuilder {
        <WhoamiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhoamiResponseBuilder {
    organization: Option<WhoamiOrganization>,
    principal: Option<WhoamiResponsePrincipal>,
    workspace: Option<UserIdentityWorkspace>,
}

impl WhoamiResponseBuilder {
    pub fn organization(mut self, value: WhoamiOrganization) -> Self {
        self.organization = Some(value);
        self
    }

    pub fn principal(mut self, value: WhoamiResponsePrincipal) -> Self {
        self.principal = Some(value);
        self
    }

    pub fn workspace(mut self, value: UserIdentityWorkspace) -> Self {
        self.workspace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhoamiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`principal`](WhoamiResponseBuilder::principal)
    pub fn build(self) -> Result<WhoamiResponse, BuildError> {
        Ok(WhoamiResponse {
            organization: self.organization,
            principal: self
                .principal
                .ok_or_else(|| BuildError::missing_field("principal"))?,
            workspace: self.workspace,
        })
    }
}
