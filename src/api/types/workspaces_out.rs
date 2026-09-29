pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspacesOut {
    /// Workspaces on this page.
    #[serde(default)]
    pub items: Vec<WorkspaceOut>,
    /// Type of paginated API object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ApiObjectType>,
    /// Page number returned.
    #[serde(default)]
    pub page: i64,
    /// Maximum number of results per page.
    #[serde(default)]
    pub page_size: i64,
    /// Total number of Workspaces that match the request.
    #[serde(default)]
    pub total: i64,
}

impl WorkspacesOut {
    pub fn builder() -> WorkspacesOutBuilder {
        <WorkspacesOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspacesOutBuilder {
    items: Option<Vec<WorkspaceOut>>,
    object: Option<ApiObjectType>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl WorkspacesOutBuilder {
    pub fn items(mut self, value: Vec<WorkspaceOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn object(mut self, value: ApiObjectType) -> Self {
        self.object = Some(value);
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

    /// Consumes the builder and constructs a [`WorkspacesOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](WorkspacesOutBuilder::items)
    /// - [`page`](WorkspacesOutBuilder::page)
    /// - [`page_size`](WorkspacesOutBuilder::page_size)
    /// - [`total`](WorkspacesOutBuilder::total)
    pub fn build(self) -> Result<WorkspacesOut, BuildError> {
        Ok(WorkspacesOut {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            object: self.object,
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
