pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptionStreamEvents {
    pub event: TranscriptionStreamEventTypes,
    pub data: TranscriptionStreamEventsData,
}

impl TranscriptionStreamEvents {
    pub fn builder() -> TranscriptionStreamEventsBuilder {
        <TranscriptionStreamEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionStreamEventsBuilder {
    event: Option<TranscriptionStreamEventTypes>,
    data: Option<TranscriptionStreamEventsData>,
}

impl TranscriptionStreamEventsBuilder {
    pub fn event(mut self, value: TranscriptionStreamEventTypes) -> Self {
        self.event = Some(value);
        self
    }

    pub fn data(mut self, value: TranscriptionStreamEventsData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionStreamEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](TranscriptionStreamEventsBuilder::event)
    /// - [`data`](TranscriptionStreamEventsBuilder::data)
    pub fn build(self) -> Result<TranscriptionStreamEvents, BuildError> {
        Ok(TranscriptionStreamEvents {
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
