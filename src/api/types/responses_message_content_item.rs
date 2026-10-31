pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesMessageContentItem {
    #[serde(rename = "input_text")]
    #[non_exhaustive]
    InputText {
        #[serde(flatten)]
        data: ResponsesInputText,
    },

    #[serde(rename = "input_image")]
    #[non_exhaustive]
    InputImage {
        #[serde(flatten)]
        data: ResponsesInputImage,
    },

    #[serde(rename = "input_file")]
    #[non_exhaustive]
    InputFile {
        #[serde(flatten)]
        data: ResponsesInputFile,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesMessageContentItem {
    pub fn input_text(data: ResponsesInputText) -> Self {
        Self::InputText { data }
    }

    pub fn input_image(data: ResponsesInputImage) -> Self {
        Self::InputImage { data }
    }

    pub fn input_file(data: ResponsesInputFile) -> Self {
        Self::InputFile { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
