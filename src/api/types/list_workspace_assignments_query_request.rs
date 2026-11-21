pub use crate::prelude::*;

/// Query parameters for list_workspace_assignments
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWorkspaceAssignmentsQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListWorkspaceAssignmentsQueryRequest {
    pub fn builder() -> ListWorkspaceAssignmentsQueryRequestBuilder {
        <ListWorkspaceAssignmentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkspaceAssignmentsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListWorkspaceAssignmentsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWorkspaceAssignmentsQueryRequest`].
    pub fn build(self) -> Result<ListWorkspaceAssignmentsQueryRequest, BuildError> {
        Ok(ListWorkspaceAssignmentsQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
