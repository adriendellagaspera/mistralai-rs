pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionEvent {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub correlation_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub extra_fields: HashMap<String, Option<ChatCompletionEventExtraFieldsValue>>,
    #[serde(default)]
    pub nb_input_tokens: i64,
    #[serde(default)]
    pub nb_output_tokens: i64,
    #[serde(default)]
    pub enabled_tools: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub request_messages: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub response_messages: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub nb_messages: i64,
    #[serde(default)]
    pub chat_transcription_events: Vec<ChatTranscriptionEvent>,
}

impl ChatCompletionEvent {
    pub fn builder() -> ChatCompletionEventBuilder {
        <ChatCompletionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionEventBuilder {
    event_id: Option<String>,
    correlation_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    extra_fields: Option<HashMap<String, Option<ChatCompletionEventExtraFieldsValue>>>,
    nb_input_tokens: Option<i64>,
    nb_output_tokens: Option<i64>,
    enabled_tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    request_messages: Option<Vec<HashMap<String, serde_json::Value>>>,
    response_messages: Option<Vec<HashMap<String, serde_json::Value>>>,
    nb_messages: Option<i64>,
    chat_transcription_events: Option<Vec<ChatTranscriptionEvent>>,
}

impl ChatCompletionEventBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn extra_fields(
        mut self,
        value: HashMap<String, Option<ChatCompletionEventExtraFieldsValue>>,
    ) -> Self {
        self.extra_fields = Some(value);
        self
    }

    pub fn nb_input_tokens(mut self, value: i64) -> Self {
        self.nb_input_tokens = Some(value);
        self
    }

    pub fn nb_output_tokens(mut self, value: i64) -> Self {
        self.nb_output_tokens = Some(value);
        self
    }

    pub fn enabled_tools(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.enabled_tools = Some(value);
        self
    }

    pub fn request_messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.request_messages = Some(value);
        self
    }

    pub fn response_messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.response_messages = Some(value);
        self
    }

    pub fn nb_messages(mut self, value: i64) -> Self {
        self.nb_messages = Some(value);
        self
    }

    pub fn chat_transcription_events(mut self, value: Vec<ChatTranscriptionEvent>) -> Self {
        self.chat_transcription_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ChatCompletionEventBuilder::event_id)
    /// - [`correlation_id`](ChatCompletionEventBuilder::correlation_id)
    /// - [`created_at`](ChatCompletionEventBuilder::created_at)
    /// - [`extra_fields`](ChatCompletionEventBuilder::extra_fields)
    /// - [`nb_input_tokens`](ChatCompletionEventBuilder::nb_input_tokens)
    /// - [`nb_output_tokens`](ChatCompletionEventBuilder::nb_output_tokens)
    /// - [`enabled_tools`](ChatCompletionEventBuilder::enabled_tools)
    /// - [`request_messages`](ChatCompletionEventBuilder::request_messages)
    /// - [`response_messages`](ChatCompletionEventBuilder::response_messages)
    /// - [`nb_messages`](ChatCompletionEventBuilder::nb_messages)
    /// - [`chat_transcription_events`](ChatCompletionEventBuilder::chat_transcription_events)
    pub fn build(self) -> Result<ChatCompletionEvent, BuildError> {
        Ok(ChatCompletionEvent {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            correlation_id: self
                .correlation_id
                .ok_or_else(|| BuildError::missing_field("correlation_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            extra_fields: self
                .extra_fields
                .ok_or_else(|| BuildError::missing_field("extra_fields"))?,
            nb_input_tokens: self
                .nb_input_tokens
                .ok_or_else(|| BuildError::missing_field("nb_input_tokens"))?,
            nb_output_tokens: self
                .nb_output_tokens
                .ok_or_else(|| BuildError::missing_field("nb_output_tokens"))?,
            enabled_tools: self
                .enabled_tools
                .ok_or_else(|| BuildError::missing_field("enabled_tools"))?,
            request_messages: self
                .request_messages
                .ok_or_else(|| BuildError::missing_field("request_messages"))?,
            response_messages: self
                .response_messages
                .ok_or_else(|| BuildError::missing_field("response_messages"))?,
            nb_messages: self
                .nb_messages
                .ok_or_else(|| BuildError::missing_field("nb_messages"))?,
            chat_transcription_events: self
                .chat_transcription_events
                .ok_or_else(|| BuildError::missing_field("chat_transcription_events"))?,
        })
    }
}
