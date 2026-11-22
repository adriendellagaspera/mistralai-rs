pub use crate::prelude::*;

/// Query parameters for list_sessions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSessionsQueryRequest {
    /// Maximum number of sessions to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListSessionsQueryRequest {
    pub fn builder() -> ListSessionsQueryRequestBuilder {
        <ListSessionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSessionsQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl ListSessionsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSessionsQueryRequest`].
    pub fn build(self) -> Result<ListSessionsQueryRequest, BuildError> {
        Ok(ListSessionsQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
        })
    }
}
