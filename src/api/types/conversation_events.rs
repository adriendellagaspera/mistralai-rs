pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationEvents {
    pub event: SseTypes,
    pub data: ConversationEventsData,
}

impl ConversationEvents {
    pub fn builder() -> ConversationEventsBuilder {
        <ConversationEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEventsBuilder {
    event: Option<SseTypes>,
    data: Option<ConversationEventsData>,
}

impl ConversationEventsBuilder {
    pub fn event(mut self, value: SseTypes) -> Self {
        self.event = Some(value);
        self
    }

    pub fn data(mut self, value: ConversationEventsData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](ConversationEventsBuilder::event)
    /// - [`data`](ConversationEventsBuilder::data)
    pub fn build(self) -> Result<ConversationEvents, BuildError> {
        Ok(ConversationEvents {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
