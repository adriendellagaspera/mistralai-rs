pub use crate::prelude::*;

/// Query parameters for list_trials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTrialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListTrialsQueryRequest {
    pub fn builder() -> ListTrialsQueryRequestBuilder {
        <ListTrialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTrialsQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListTrialsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTrialsQueryRequest`].
    pub fn build(self) -> Result<ListTrialsQueryRequest, BuildError> {
        Ok(ListTrialsQueryRequest {
            page_size: self.page_size,
            page: self.page,
        })
    }
}
