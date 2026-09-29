pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupWorkspaceAssignmentsOut {
    /// Workspace assignments on this page.
    #[serde(default)]
    pub items: Vec<GroupWorkspaceAssignmentOut>,
    /// Page number returned.
    #[serde(default)]
    pub page: i64,
    /// Maximum number of results per page.
    #[serde(default)]
    pub page_size: i64,
    /// Total number of Workspace assignments that match the request.
    #[serde(default)]
    pub total: i64,
}

impl GroupWorkspaceAssignmentsOut {
    pub fn builder() -> GroupWorkspaceAssignmentsOutBuilder {
        <GroupWorkspaceAssignmentsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupWorkspaceAssignmentsOutBuilder {
    items: Option<Vec<GroupWorkspaceAssignmentOut>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl GroupWorkspaceAssignmentsOutBuilder {
    pub fn items(mut self, value: Vec<GroupWorkspaceAssignmentOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupWorkspaceAssignmentsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](GroupWorkspaceAssignmentsOutBuilder::items)
    /// - [`page`](GroupWorkspaceAssignmentsOutBuilder::page)
    /// - [`page_size`](GroupWorkspaceAssignmentsOutBuilder::page_size)
    /// - [`total`](GroupWorkspaceAssignmentsOutBuilder::total)
    pub fn build(self) -> Result<GroupWorkspaceAssignmentsOut, BuildError> {
        Ok(GroupWorkspaceAssignmentsOut {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
