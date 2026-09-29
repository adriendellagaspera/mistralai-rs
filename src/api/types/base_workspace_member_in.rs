pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BaseWorkspaceMemberIn {
    /// User ID of the Workspace member.
    #[serde(default)]
    pub user_uuid: String,
}

impl BaseWorkspaceMemberIn {
    pub fn builder() -> BaseWorkspaceMemberInBuilder {
        <BaseWorkspaceMemberInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BaseWorkspaceMemberInBuilder {
    user_uuid: Option<String>,
}

impl BaseWorkspaceMemberInBuilder {
    pub fn user_uuid(mut self, value: impl Into<String>) -> Self {
        self.user_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BaseWorkspaceMemberIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_uuid`](BaseWorkspaceMemberInBuilder::user_uuid)
    pub fn build(self) -> Result<BaseWorkspaceMemberIn, BuildError> {
        Ok(BaseWorkspaceMemberIn {
            user_uuid: self
                .user_uuid
                .ok_or_else(|| BuildError::missing_field("user_uuid"))?,
        })
    }
}
