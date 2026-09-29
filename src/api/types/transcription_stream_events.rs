pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptionStreamEvents {
    pub data: TranscriptionStreamEventsData,
    pub event: TranscriptionStreamEventTypes,
}

impl TranscriptionStreamEvents {
    pub fn builder() -> TranscriptionStreamEventsBuilder {
        <TranscriptionStreamEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionStreamEventsBuilder {
    data: Option<TranscriptionStreamEventsData>,
    event: Option<TranscriptionStreamEventTypes>,
}

impl TranscriptionStreamEventsBuilder {
    pub fn data(mut self, value: TranscriptionStreamEventsData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: TranscriptionStreamEventTypes) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionStreamEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](TranscriptionStreamEventsBuilder::data)
    /// - [`event`](TranscriptionStreamEventsBuilder::event)
    pub fn build(self) -> Result<TranscriptionStreamEvents, BuildError> {
        Ok(TranscriptionStreamEvents {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
