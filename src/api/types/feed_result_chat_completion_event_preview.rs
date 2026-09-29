pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultChatCompletionEventPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChatCompletionEventPreview>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FeedResultChatCompletionEventPreview {
    pub fn builder() -> FeedResultChatCompletionEventPreviewBuilder {
        <FeedResultChatCompletionEventPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultChatCompletionEventPreviewBuilder {
    results: Option<Vec<ChatCompletionEventPreview>>,
    next: Option<String>,
    cursor: Option<String>,
}

impl FeedResultChatCompletionEventPreviewBuilder {
    pub fn results(mut self, value: Vec<ChatCompletionEventPreview>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedResultChatCompletionEventPreview`].
    pub fn build(self) -> Result<FeedResultChatCompletionEventPreview, BuildError> {
        Ok(FeedResultChatCompletionEventPreview {
            results: self.results,
            next: self.next,
            cursor: self.cursor,
        })
    }
}
