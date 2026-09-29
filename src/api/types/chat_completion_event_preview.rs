pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionEventPreview {
    #[serde(default)]
    pub correlation_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub extra_fields: HashMap<String, Option<ChatCompletionEventPreviewExtraFieldsValue>>,
    #[serde(default)]
    pub nb_input_tokens: i64,
    #[serde(default)]
    pub nb_output_tokens: i64,
}

impl ChatCompletionEventPreview {
    pub fn builder() -> ChatCompletionEventPreviewBuilder {
        <ChatCompletionEventPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionEventPreviewBuilder {
    correlation_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    event_id: Option<String>,
    extra_fields: Option<HashMap<String, Option<ChatCompletionEventPreviewExtraFieldsValue>>>,
    nb_input_tokens: Option<i64>,
    nb_output_tokens: Option<i64>,
}

impl ChatCompletionEventPreviewBuilder {
    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn extra_fields(
        mut self,
        value: HashMap<String, Option<ChatCompletionEventPreviewExtraFieldsValue>>,
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

    /// Consumes the builder and constructs a [`ChatCompletionEventPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`correlation_id`](ChatCompletionEventPreviewBuilder::correlation_id)
    /// - [`created_at`](ChatCompletionEventPreviewBuilder::created_at)
    /// - [`event_id`](ChatCompletionEventPreviewBuilder::event_id)
    /// - [`extra_fields`](ChatCompletionEventPreviewBuilder::extra_fields)
    /// - [`nb_input_tokens`](ChatCompletionEventPreviewBuilder::nb_input_tokens)
    /// - [`nb_output_tokens`](ChatCompletionEventPreviewBuilder::nb_output_tokens)
    pub fn build(self) -> Result<ChatCompletionEventPreview, BuildError> {
        Ok(ChatCompletionEventPreview {
            correlation_id: self
                .correlation_id
                .ok_or_else(|| BuildError::missing_field("correlation_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            extra_fields: self
                .extra_fields
                .ok_or_else(|| BuildError::missing_field("extra_fields"))?,
            nb_input_tokens: self
                .nb_input_tokens
                .ok_or_else(|| BuildError::missing_field("nb_input_tokens"))?,
            nb_output_tokens: self
                .nb_output_tokens
                .ok_or_else(|| BuildError::missing_field("nb_output_tokens"))?,
        })
    }
}
