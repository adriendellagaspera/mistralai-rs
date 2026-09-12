use std::pin::Pin;

use futures_util::{Stream, StreamExt};

use super::SdkError;
use crate::generated::client::HttpClient;
use crate::generated::types::{
    AssistantMessage, AssistantMessageContent, ChatCompletionRequest,
    ChatCompletionRequestMessagesItemUnion, ChatCompletionResponse, CompletionChunk,
    DeltaMessageContent, SystemMessage, SystemMessageContent, UserMessage, UserMessageContent,
};
use crate::streaming;

/// A chat message with textual content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self::System(content.into())
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::User(content.into())
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::Assistant(content.into())
    }
}

impl From<Message> for ChatCompletionRequestMessagesItemUnion {
    fn from(message: Message) -> Self {
        match message {
            Message::System(content) => Self::SystemMessage(SystemMessage {
                content: SystemMessageContent::String(content),
                role: None,
            }),
            Message::User(content) => Self::UserMessage(UserMessage {
                content: Some(UserMessageContent::String(content)),
                role: None,
            }),
            Message::Assistant(content) => Self::AssistantMessage(AssistantMessage {
                content: Some(Some(AssistantMessageContent::String(content))),
                prefix: None,
                role: None,
                tool_calls: None,
            }),
        }
    }
}

/// A chat completion request.
///
/// Common options have fluent setters. ChatRequest::from_raw remains an
/// explicit escape hatch for less common OpenAPI fields during the facade's
/// incremental rollout.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    raw: ChatCompletionRequest,
}

impl ChatRequest {
    pub fn new(model: impl Into<String>, messages: impl IntoIterator<Item = Message>) -> Self {
        Self {
            raw: ChatCompletionRequest::new(
                messages.into_iter().map(Into::into).collect(),
                model.into(),
            ),
        }
    }

    pub fn from_raw(raw: ChatCompletionRequest) -> Self {
        Self { raw }
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.raw.max_tokens = Some(Some(i64::from(max_tokens)));
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.raw.temperature = Some(Some(temperature));
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.raw.top_p = Some(top_p);
        self
    }

    pub fn random_seed(mut self, random_seed: i64) -> Self {
        self.raw.random_seed = Some(Some(random_seed));
        self
    }

    pub fn safe_prompt(mut self, safe_prompt: bool) -> Self {
        self.raw.safe_prompt = Some(safe_prompt);
        self
    }

    pub fn as_raw(&self) -> &ChatCompletionRequest {
        &self.raw
    }

    pub fn into_raw(self) -> ChatCompletionRequest {
        self.raw
    }

    fn into_raw_with_stream(mut self, stream: bool) -> ChatCompletionRequest {
        self.raw.stream = Some(stream);
        self.raw
    }
}

/// A stable facade over the generated chat completion response.
#[derive(Debug, Clone)]
pub struct ChatResponse {
    raw: ChatCompletionResponse,
}

impl ChatResponse {
    /// Text from the first choice, when that choice contains plain text.
    pub fn text(&self) -> Option<&str> {
        let content = self
            .raw
            .choices
            .first()?
            .message
            .content
            .as_ref()?
            .as_ref()?;
        match content {
            AssistantMessageContent::String(text) => Some(text),
            AssistantMessageContent::ContentChunkArray(_) => None,
        }
    }

    pub fn raw(&self) -> &ChatCompletionResponse {
        &self.raw
    }

    pub fn into_raw(self) -> ChatCompletionResponse {
        self.raw
    }
}

impl From<ChatCompletionResponse> for ChatResponse {
    fn from(raw: ChatCompletionResponse) -> Self {
        Self { raw }
    }
}

/// A stable facade over one generated streaming chunk.
#[derive(Debug, Clone)]
pub struct ChatStreamChunk {
    raw: CompletionChunk,
}

impl ChatStreamChunk {
    /// Text delta from the first choice, when that delta contains plain text.
    pub fn text(&self) -> Option<&str> {
        let content = self.raw.choices.first()?.delta.content.as_ref()?.as_ref()?;
        match content {
            DeltaMessageContent::String(text) => Some(text),
            DeltaMessageContent::ContentChunkArrayInline(_) => None,
        }
    }

    pub fn raw(&self) -> &CompletionChunk {
        &self.raw
    }

    pub fn into_raw(self) -> CompletionChunk {
        self.raw
    }
}

impl From<CompletionChunk> for ChatStreamChunk {
    fn from(raw: CompletionChunk) -> Self {
        Self { raw }
    }
}

/// Owned chat stream whose lifetime is independent of the temporary chat()
/// resource used to start it.
pub type ChatStream =
    Pin<Box<dyn Stream<Item = Result<ChatStreamChunk, SdkError>> + Send + 'static>>;

/// Chat resource, matching the taxonomy of Mistral's official SDKs.
#[derive(Clone, Copy)]
pub struct Chat<'a> {
    raw: &'a HttpClient,
}

impl<'a> Chat<'a> {
    pub(crate) fn new(raw: &'a HttpClient) -> Self {
        Self { raw }
    }

    /// Create a chat completion.
    pub async fn complete(&self, request: ChatRequest) -> Result<ChatResponse, SdkError> {
        self.raw
            .chat_completion_v1_chat_completions_post(request.into_raw_with_stream(false))
            .await
            .map(ChatResponse::from)
            .map_err(SdkError::from)
    }

    /// Stream completion chunks without exposing the raw SSE byte stream.
    pub async fn stream(&self, request: ChatRequest) -> Result<ChatStream, SdkError> {
        let bytes = self
            .raw
            .chat_completion_v1_chat_completions_post_stream(request.into_raw_with_stream(true))
            .await
            .map_err(SdkError::from)?;
        let events = streaming::json_events::<_, _, CompletionChunk>(bytes).map(|event| {
            event
                .map(|event| ChatStreamChunk::from(event.data))
                .map_err(SdkError::from)
        });
        Ok(Box::pin(events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_maps_directly_to_generated_chat_contract() {
        let raw = ChatRequest::new(
            "mistral-small-latest",
            [Message::system("Be concise."), Message::user("Bonjour")],
        )
        .max_tokens(64)
        .temperature(0.2)
        .into_raw_with_stream(false);

        let value = serde_json::to_value(raw).unwrap();
        assert_eq!(value["model"], "mistral-small-latest");
        assert_eq!(value["messages"][0]["role"], "system");
        assert_eq!(value["messages"][1]["content"], "Bonjour");
        assert_eq!(value["max_tokens"], 64);
        assert_eq!(value["temperature"], 0.2);
        assert_eq!(value["stream"], false);
    }

    #[test]
    fn streaming_request_overrides_the_raw_stream_flag() {
        let mut generated = ChatCompletionRequest::new(Vec::new(), "model".into());
        generated.stream = Some(false);
        let raw = ChatRequest::from_raw(generated).into_raw_with_stream(true);
        assert_eq!(raw.stream, Some(true));
    }
}
