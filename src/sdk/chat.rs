use serde::Serialize;

use super::SdkError;
use crate::generated::client::HttpClient;
use crate::generated::types::{ChatCompletionRequest, ChatCompletionResponse};

pub type ChatResponse = ChatCompletionResponse;

/// A chat message with a role and textual content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "role", content = "content", rename_all = "lowercase")]
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

/// Ergonomic request for a non-streaming chat completion.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u64>,
}

impl ChatRequest {
    pub fn new(model: impl Into<String>, messages: impl IntoIterator<Item = Message>) -> Self {
        Self {
            model: model.into(),
            messages: messages.into_iter().collect(),
            max_tokens: None,
        }
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    fn into_raw(self) -> Result<ChatCompletionRequest, SdkError> {
        let mut value = serde_json::json!({
            "model": self.model,
            "messages": self.messages,
            "stream": false,
        });
        if let Some(max_tokens) = self.max_tokens {
            value["max_tokens"] = serde_json::json!(max_tokens);
        }
        Ok(serde_json::from_value(value)?)
    }
}

/// Chat resource, matching the taxonomy of Mistral's official SDKs.
#[derive(Debug, Clone, Copy)]
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
            .chat_completion_v1_chat_completions_post(request.into_raw()?)
            .await
            .map_err(SdkError::api)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_maps_to_generated_chat_contract() {
        let raw = ChatRequest::new(
            "mistral-small-latest",
            [Message::system("Be concise."), Message::user("Bonjour")],
        )
        .max_tokens(64)
        .into_raw()
        .unwrap();

        let value = serde_json::to_value(raw).unwrap();
        assert_eq!(value["model"], "mistral-small-latest");
        assert_eq!(value["messages"][0]["role"], "system");
        assert_eq!(value["messages"][1]["content"], "Bonjour");
        assert_eq!(value["max_tokens"], 64);
        assert_eq!(value["stream"], false);
    }
}
