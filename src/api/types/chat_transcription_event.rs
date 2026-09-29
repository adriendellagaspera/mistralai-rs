pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatTranscriptionEvent {
    #[serde(default)]
    pub audio_url: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub response_message: HashMap<String, serde_json::Value>,
}

impl ChatTranscriptionEvent {
    pub fn builder() -> ChatTranscriptionEventBuilder {
        <ChatTranscriptionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatTranscriptionEventBuilder {
    audio_url: Option<String>,
    model: Option<String>,
    response_message: Option<HashMap<String, serde_json::Value>>,
}

impl ChatTranscriptionEventBuilder {
    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn response_message(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatTranscriptionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_url`](ChatTranscriptionEventBuilder::audio_url)
    /// - [`model`](ChatTranscriptionEventBuilder::model)
    /// - [`response_message`](ChatTranscriptionEventBuilder::response_message)
    pub fn build(self) -> Result<ChatTranscriptionEvent, BuildError> {
        Ok(ChatTranscriptionEvent {
            audio_url: self
                .audio_url
                .ok_or_else(|| BuildError::missing_field("audio_url"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            response_message: self
                .response_message
                .ok_or_else(|| BuildError::missing_field("response_message"))?,
        })
    }
}
