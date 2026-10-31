pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesInputFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ResponsesInputFileDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesInputFile {
    pub fn builder() -> ResponsesInputFileBuilder {
        <ResponsesInputFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesInputFileBuilder {
    detail: Option<ResponsesInputFileDetail>,
    file_data: Option<String>,
    file_id: Option<String>,
    file_url: Option<String>,
    filename: Option<String>,
    prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
}

impl ResponsesInputFileBuilder {
    pub fn detail(mut self, value: ResponsesInputFileDetail) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn file_data(mut self, value: impl Into<String>) -> Self {
        self.file_data = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn prompt_cache_breakpoint(mut self, value: ResponsesPromptCacheBreakpoint) -> Self {
        self.prompt_cache_breakpoint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesInputFile`].
    pub fn build(self) -> Result<ResponsesInputFile, BuildError> {
        Ok(ResponsesInputFile {
            detail: self.detail,
            file_data: self.file_data,
            file_id: self.file_id,
            file_url: self.file_url,
            filename: self.filename,
            prompt_cache_breakpoint: self.prompt_cache_breakpoint,
            extra: Default::default(),
        })
    }
}
