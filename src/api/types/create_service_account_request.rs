pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Workspace role UUIDs to assign. Omitted or empty lists default to member, which grants no role permissions. Workspace-wide sharing may still grant access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub workspace_id: String,
}

impl CreateServiceAccountRequest {
    pub fn builder() -> CreateServiceAccountRequestBuilder {
        <CreateServiceAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceAccountRequestBuilder {
    description: Option<String>,
    name: Option<String>,
    role_ids: Option<Vec<String>>,
    workspace_id: Option<String>,
}

impl CreateServiceAccountRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role_ids(mut self, value: Vec<String>) -> Self {
        self.role_ids = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateServiceAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateServiceAccountRequestBuilder::name)
    /// - [`workspace_id`](CreateServiceAccountRequestBuilder::workspace_id)
    pub fn build(self) -> Result<CreateServiceAccountRequest, BuildError> {
        Ok(CreateServiceAccountRequest {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role_ids: self.role_ids,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
