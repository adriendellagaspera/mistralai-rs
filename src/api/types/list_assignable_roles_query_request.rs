pub use crate::prelude::*;

/// Query parameters for list_assignable_roles
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAssignableRolesQueryRequest {
    #[serde(default)]
    pub workspace_id: String,
}

impl ListAssignableRolesQueryRequest {
    pub fn builder() -> ListAssignableRolesQueryRequestBuilder {
        <ListAssignableRolesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAssignableRolesQueryRequestBuilder {
    workspace_id: Option<String>,
}

impl ListAssignableRolesQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAssignableRolesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](ListAssignableRolesQueryRequestBuilder::workspace_id)
    pub fn build(self) -> Result<ListAssignableRolesQueryRequest, BuildError> {
        Ok(ListAssignableRolesQueryRequest {
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
