pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchChatCompletionEventIdsResponse {
    #[serde(default)]
    pub completion_event_ids: Vec<String>,
}

impl SearchChatCompletionEventIdsResponse {
    pub fn builder() -> SearchChatCompletionEventIdsResponseBuilder {
        <SearchChatCompletionEventIdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchChatCompletionEventIdsResponseBuilder {
    completion_event_ids: Option<Vec<String>>,
}

impl SearchChatCompletionEventIdsResponseBuilder {
    pub fn completion_event_ids(mut self, value: Vec<String>) -> Self {
        self.completion_event_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchChatCompletionEventIdsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completion_event_ids`](SearchChatCompletionEventIdsResponseBuilder::completion_event_ids)
    pub fn build(self) -> Result<SearchChatCompletionEventIdsResponse, BuildError> {
        Ok(SearchChatCompletionEventIdsResponse {
            completion_event_ids: self
                .completion_event_ids
                .ok_or_else(|| BuildError::missing_field("completion_event_ids"))?,
        })
    }
}
