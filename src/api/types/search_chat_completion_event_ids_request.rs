pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchChatCompletionEventIdsRequest {
    #[serde(default)]
    pub search_params: FilterPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
}

impl SearchChatCompletionEventIdsRequest {
    pub fn builder() -> SearchChatCompletionEventIdsRequestBuilder {
        <SearchChatCompletionEventIdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchChatCompletionEventIdsRequestBuilder {
    search_params: Option<FilterPayload>,
    extra_fields: Option<Vec<String>>,
}

impl SearchChatCompletionEventIdsRequestBuilder {
    pub fn search_params(mut self, value: FilterPayload) -> Self {
        self.search_params = Some(value);
        self
    }

    pub fn extra_fields(mut self, value: Vec<String>) -> Self {
        self.extra_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchChatCompletionEventIdsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`search_params`](SearchChatCompletionEventIdsRequestBuilder::search_params)
    pub fn build(self) -> Result<SearchChatCompletionEventIdsRequest, BuildError> {
        Ok(SearchChatCompletionEventIdsRequest {
            search_params: self
                .search_params
                .ok_or_else(|| BuildError::missing_field("search_params"))?,
            extra_fields: self.extra_fields,
        })
    }
}
