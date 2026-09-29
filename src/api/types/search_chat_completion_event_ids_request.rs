pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchChatCompletionEventIdsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<Vec<String>>,
    #[serde(default)]
    pub search_params: FilterPayload,
}

impl SearchChatCompletionEventIdsRequest {
    pub fn builder() -> SearchChatCompletionEventIdsRequestBuilder {
        <SearchChatCompletionEventIdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchChatCompletionEventIdsRequestBuilder {
    extra_fields: Option<Vec<String>>,
    search_params: Option<FilterPayload>,
}

impl SearchChatCompletionEventIdsRequestBuilder {
    pub fn extra_fields(mut self, value: Vec<String>) -> Self {
        self.extra_fields = Some(value);
        self
    }

    pub fn search_params(mut self, value: FilterPayload) -> Self {
        self.search_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchChatCompletionEventIdsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`search_params`](SearchChatCompletionEventIdsRequestBuilder::search_params)
    pub fn build(self) -> Result<SearchChatCompletionEventIdsRequest, BuildError> {
        Ok(SearchChatCompletionEventIdsRequest {
            extra_fields: self.extra_fields,
            search_params: self
                .search_params
                .ok_or_else(|| BuildError::missing_field("search_params"))?,
        })
    }
}
