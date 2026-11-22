pub use crate::prelude::*;

/// Query parameters for list_tasks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTasksQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListTasksQueryRequest {
    pub fn builder() -> ListTasksQueryRequestBuilder {
        <ListTasksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTasksQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListTasksQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTasksQueryRequest`].
    pub fn build(self) -> Result<ListTasksQueryRequest, BuildError> {
        Ok(ListTasksQueryRequest {
            page_size: self.page_size,
            page: self.page,
        })
    }
}
