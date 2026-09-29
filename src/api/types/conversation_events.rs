pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationEvents {
    pub data: ConversationEventsData,
    pub event: SseTypes,
}

impl ConversationEvents {
    pub fn builder() -> ConversationEventsBuilder {
        <ConversationEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEventsBuilder {
    data: Option<ConversationEventsData>,
    event: Option<SseTypes>,
}

impl ConversationEventsBuilder {
    pub fn data(mut self, value: ConversationEventsData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: SseTypes) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ConversationEventsBuilder::data)
    /// - [`event`](ConversationEventsBuilder::event)
    pub fn build(self) -> Result<ConversationEvents, BuildError> {
        Ok(ConversationEvents {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
