pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchChatCompletionEventsResponse {
    #[serde(default)]
    pub completion_events: FeedResultChatCompletionEventPreview,
}

impl SearchChatCompletionEventsResponse {
    pub fn builder() -> SearchChatCompletionEventsResponseBuilder {
        <SearchChatCompletionEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchChatCompletionEventsResponseBuilder {
    completion_events: Option<FeedResultChatCompletionEventPreview>,
}

impl SearchChatCompletionEventsResponseBuilder {
    pub fn completion_events(mut self, value: FeedResultChatCompletionEventPreview) -> Self {
        self.completion_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchChatCompletionEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completion_events`](SearchChatCompletionEventsResponseBuilder::completion_events)
    pub fn build(self) -> Result<SearchChatCompletionEventsResponse, BuildError> {
        Ok(SearchChatCompletionEventsResponse {
            completion_events: self
                .completion_events
                .ok_or_else(|| BuildError::missing_field("completion_events"))?,
        })
    }
}
