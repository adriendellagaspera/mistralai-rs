pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesFunctionCallOutputOutputOneItem {
    #[serde(rename = "input_text")]
    #[non_exhaustive]
    InputText {
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "input_image")]
    #[non_exhaustive]
    InputImage {
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<ResponsesInputImageContentDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    },

    #[serde(rename = "input_file")]
    #[non_exhaustive]
    InputFile {
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<ResponsesInputFileContentDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesFunctionCallOutputOutputOneItem {
    pub fn input_text(text: String) -> Self {
        Self::InputText {
            prompt_cache_breakpoint: None,
            text,
        }
    }

    pub fn input_image() -> Self {
        Self::InputImage {
            detail: None,
            file_id: None,
            image_url: None,
            prompt_cache_breakpoint: None,
        }
    }

    pub fn input_file() -> Self {
        Self::InputFile {
            detail: None,
            file_data: None,
            file_id: None,
            file_url: None,
            filename: None,
            prompt_cache_breakpoint: None,
        }
    }

    pub fn input_text_with_prompt_cache_breakpoint(
        prompt_cache_breakpoint: ResponsesPromptCacheBreakpoint,
        text: String,
    ) -> Self {
        Self::InputText {
            prompt_cache_breakpoint: Some(prompt_cache_breakpoint),
            text,
        }
    }

    pub fn input_image_with_detail(
        detail: ResponsesInputImageContentDetail,
        file_id: Option<String>,
        image_url: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputImage {
            detail: Some(detail),
            file_id,
            image_url,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_image_with_file_id(
        detail: Option<ResponsesInputImageContentDetail>,
        file_id: String,
        image_url: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputImage {
            detail,
            file_id: Some(file_id),
            image_url,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_image_with_image_url(
        detail: Option<ResponsesInputImageContentDetail>,
        file_id: Option<String>,
        image_url: String,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputImage {
            detail,
            file_id,
            image_url: Some(image_url),
            prompt_cache_breakpoint,
        }
    }

    pub fn input_image_with_prompt_cache_breakpoint(
        detail: Option<ResponsesInputImageContentDetail>,
        file_id: Option<String>,
        image_url: Option<String>,
        prompt_cache_breakpoint: ResponsesPromptCacheBreakpoint,
    ) -> Self {
        Self::InputImage {
            detail,
            file_id,
            image_url,
            prompt_cache_breakpoint: Some(prompt_cache_breakpoint),
        }
    }

    pub fn input_file_with_detail(
        detail: ResponsesInputFileContentDetail,
        file_data: Option<String>,
        file_id: Option<String>,
        file_url: Option<String>,
        filename: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputFile {
            detail: Some(detail),
            file_data,
            file_id,
            file_url,
            filename,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_file_with_file_data(
        detail: Option<ResponsesInputFileContentDetail>,
        file_data: String,
        file_id: Option<String>,
        file_url: Option<String>,
        filename: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputFile {
            detail,
            file_data: Some(file_data),
            file_id,
            file_url,
            filename,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_file_with_file_id(
        detail: Option<ResponsesInputFileContentDetail>,
        file_data: Option<String>,
        file_id: String,
        file_url: Option<String>,
        filename: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputFile {
            detail,
            file_data,
            file_id: Some(file_id),
            file_url,
            filename,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_file_with_file_url(
        detail: Option<ResponsesInputFileContentDetail>,
        file_data: Option<String>,
        file_id: Option<String>,
        file_url: String,
        filename: Option<String>,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputFile {
            detail,
            file_data,
            file_id,
            file_url: Some(file_url),
            filename,
            prompt_cache_breakpoint,
        }
    }

    pub fn input_file_with_filename(
        detail: Option<ResponsesInputFileContentDetail>,
        file_data: Option<String>,
        file_id: Option<String>,
        file_url: Option<String>,
        filename: String,
        prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    ) -> Self {
        Self::InputFile {
            detail,
            file_data,
            file_id,
            file_url,
            filename: Some(filename),
            prompt_cache_breakpoint,
        }
    }

    pub fn input_file_with_prompt_cache_breakpoint(
        detail: Option<ResponsesInputFileContentDetail>,
        file_data: Option<String>,
        file_id: Option<String>,
        file_url: Option<String>,
        filename: Option<String>,
        prompt_cache_breakpoint: ResponsesPromptCacheBreakpoint,
    ) -> Self {
        Self::InputFile {
            detail,
            file_data,
            file_id,
            file_url,
            filename,
            prompt_cache_breakpoint: Some(prompt_cache_breakpoint),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
