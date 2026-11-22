pub use crate::prelude::*;

/// Query parameters for text_content
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextContentQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_end: Option<i64>,
}

impl TextContentQueryRequest {
    pub fn builder() -> TextContentQueryRequestBuilder {
        <TextContentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextContentQueryRequestBuilder {
    page_start: Option<i64>,
    page_end: Option<i64>,
}

impl TextContentQueryRequestBuilder {
    pub fn page_start(mut self, value: i64) -> Self {
        self.page_start = Some(value);
        self
    }

    pub fn page_end(mut self, value: i64) -> Self {
        self.page_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextContentQueryRequest`].
    pub fn build(self) -> Result<TextContentQueryRequest, BuildError> {
        Ok(TextContentQueryRequest {
            page_start: self.page_start,
            page_end: self.page_end,
        })
    }
}
