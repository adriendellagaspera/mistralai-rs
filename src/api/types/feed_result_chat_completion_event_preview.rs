pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultChatCompletionEventPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChatCompletionEventPreview>>,
}

impl FeedResultChatCompletionEventPreview {
    pub fn builder() -> FeedResultChatCompletionEventPreviewBuilder {
        <FeedResultChatCompletionEventPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultChatCompletionEventPreviewBuilder {
    cursor: Option<String>,
    next: Option<String>,
    results: Option<Vec<ChatCompletionEventPreview>>,
}

impl FeedResultChatCompletionEventPreviewBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ChatCompletionEventPreview>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedResultChatCompletionEventPreview`].
    pub fn build(self) -> Result<FeedResultChatCompletionEventPreview, BuildError> {
        Ok(FeedResultChatCompletionEventPreview {
            cursor: self.cursor,
            next: self.next,
            results: self.results,
        })
    }
}
