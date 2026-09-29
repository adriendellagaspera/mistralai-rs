pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceRoleRef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uuid: String,
}

impl WorkspaceRoleRef {
    pub fn builder() -> WorkspaceRoleRefBuilder {
        <WorkspaceRoleRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceRoleRefBuilder {
    name: Option<String>,
    uuid: Option<String>,
}

impl WorkspaceRoleRefBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceRoleRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceRoleRefBuilder::name)
    /// - [`uuid`](WorkspaceRoleRefBuilder::uuid)
    pub fn build(self) -> Result<WorkspaceRoleRef, BuildError> {
        Ok(WorkspaceRoleRef {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
