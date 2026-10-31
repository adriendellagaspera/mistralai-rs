pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesInputImage {
    pub detail: ResponsesInputImageDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesInputImage {
    pub fn builder() -> ResponsesInputImageBuilder {
        <ResponsesInputImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesInputImageBuilder {
    detail: Option<ResponsesInputImageDetail>,
    file_id: Option<String>,
    image_url: Option<String>,
    prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
}

impl ResponsesInputImageBuilder {
    pub fn detail(mut self, value: ResponsesInputImageDetail) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn prompt_cache_breakpoint(mut self, value: ResponsesPromptCacheBreakpoint) -> Self {
        self.prompt_cache_breakpoint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesInputImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](ResponsesInputImageBuilder::detail)
    pub fn build(self) -> Result<ResponsesInputImage, BuildError> {
        Ok(ResponsesInputImage {
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            file_id: self.file_id,
            image_url: self.image_url,
            prompt_cache_breakpoint: self.prompt_cache_breakpoint,
            extra: Default::default(),
        })
    }
}
