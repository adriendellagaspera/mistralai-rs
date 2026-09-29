pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCampaignSelectedEventsResponse {
    #[serde(default)]
    pub completion_events: PaginatedResultChatCompletionEventPreview,
}

impl ListCampaignSelectedEventsResponse {
    pub fn builder() -> ListCampaignSelectedEventsResponseBuilder {
        <ListCampaignSelectedEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCampaignSelectedEventsResponseBuilder {
    completion_events: Option<PaginatedResultChatCompletionEventPreview>,
}

impl ListCampaignSelectedEventsResponseBuilder {
    pub fn completion_events(mut self, value: PaginatedResultChatCompletionEventPreview) -> Self {
        self.completion_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCampaignSelectedEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completion_events`](ListCampaignSelectedEventsResponseBuilder::completion_events)
    pub fn build(self) -> Result<ListCampaignSelectedEventsResponse, BuildError> {
        Ok(ListCampaignSelectedEventsResponse {
            completion_events: self
                .completion_events
                .ok_or_else(|| BuildError::missing_field("completion_events"))?,
        })
    }
}
