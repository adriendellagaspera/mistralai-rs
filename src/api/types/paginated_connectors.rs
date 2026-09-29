pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedConnectors {
    #[serde(default)]
    pub items: Vec<Connector>,
    #[serde(default)]
    pub pagination: PaginationResponse,
}

impl PaginatedConnectors {
    pub fn builder() -> PaginatedConnectorsBuilder {
        <PaginatedConnectorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedConnectorsBuilder {
    items: Option<Vec<Connector>>,
    pagination: Option<PaginationResponse>,
}

impl PaginatedConnectorsBuilder {
    pub fn items(mut self, value: Vec<Connector>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationResponse) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedConnectors`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PaginatedConnectorsBuilder::items)
    /// - [`pagination`](PaginatedConnectorsBuilder::pagination)
    pub fn build(self) -> Result<PaginatedConnectors, BuildError> {
        Ok(PaginatedConnectors {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
