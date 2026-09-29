pub use crate::prelude::*;

/// Query parameters for connector_list_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorListV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filters: Option<ConnectorsQueryFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ConnectorListV1QueryRequest {
    pub fn builder() -> ConnectorListV1QueryRequestBuilder {
        <ConnectorListV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorListV1QueryRequestBuilder {
    query_filters: Option<ConnectorsQueryFilters>,
    cursor: Option<String>,
    page_size: Option<i64>,
}

impl ConnectorListV1QueryRequestBuilder {
    pub fn query_filters(mut self, value: ConnectorsQueryFilters) -> Self {
        self.query_filters = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorListV1QueryRequest`].
    pub fn build(self) -> Result<ConnectorListV1QueryRequest, BuildError> {
        Ok(ConnectorListV1QueryRequest {
            query_filters: self.query_filters,
            cursor: self.cursor,
            page_size: self.page_size,
        })
    }
}
