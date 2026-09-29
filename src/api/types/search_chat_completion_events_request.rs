pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchChatCompletionEventsRequest {
    #[serde(default)]
    pub search_params: FilterPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub cursor: Option<String>,
}

impl SearchChatCompletionEventsRequest {
    pub fn builder() -> SearchChatCompletionEventsRequestBuilder {
        <SearchChatCompletionEventsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchChatCompletionEventsRequestBuilder {
    search_params: Option<FilterPayload>,
    extra_fields: Option<Vec<String>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl SearchChatCompletionEventsRequestBuilder {
    pub fn search_params(mut self, value: FilterPayload) -> Self {
        self.search_params = Some(value);
        self
    }

    pub fn extra_fields(mut self, value: Vec<String>) -> Self {
        self.extra_fields = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchChatCompletionEventsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`search_params`](SearchChatCompletionEventsRequestBuilder::search_params)
    pub fn build(self) -> Result<SearchChatCompletionEventsRequest, BuildError> {
        Ok(SearchChatCompletionEventsRequest {
            search_params: self
                .search_params
                .ok_or_else(|| BuildError::missing_field("search_params"))?,
            extra_fields: self.extra_fields,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
