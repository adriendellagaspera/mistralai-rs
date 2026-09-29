pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceIn {
    /// Updated Workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated Workspace icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Updated Workspace name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateWorkspaceIn {
    pub fn builder() -> UpdateWorkspaceInBuilder {
        <UpdateWorkspaceInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceInBuilder {
    description: Option<String>,
    icon: Option<String>,
    name: Option<String>,
}

impl UpdateWorkspaceInBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceIn`].
    pub fn build(self) -> Result<UpdateWorkspaceIn, BuildError> {
        Ok(UpdateWorkspaceIn {
            description: self.description,
            icon: self.icon,
            name: self.name,
        })
    }
}
