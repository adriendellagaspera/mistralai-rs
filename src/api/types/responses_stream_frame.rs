pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesStreamFrame {
    pub data: ResponsesStreamFrameData,
    #[serde(default)]
    pub event: String,
}

impl ResponsesStreamFrame {
    pub fn builder() -> ResponsesStreamFrameBuilder {
        <ResponsesStreamFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesStreamFrameBuilder {
    data: Option<ResponsesStreamFrameData>,
    event: Option<String>,
}

impl ResponsesStreamFrameBuilder {
    pub fn data(mut self, value: ResponsesStreamFrameData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesStreamFrame`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ResponsesStreamFrameBuilder::data)
    /// - [`event`](ResponsesStreamFrameBuilder::event)
    pub fn build(self) -> Result<ResponsesStreamFrame, BuildError> {
        Ok(ResponsesStreamFrame {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
