pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatCompletionRequestToolsItem {
    #[serde(rename = "connector")]
    #[non_exhaustive]
    Connector {
        #[serde(flatten)]
        data: CustomConnector,
    },

    #[serde(rename = "document_library")]
    #[non_exhaustive]
    DocumentLibrary {
        #[serde(flatten)]
        data: DocumentLibraryTool,
    },

    #[serde(rename = "function")]
    #[non_exhaustive]
    Function {
        #[serde(flatten)]
        data: Tool,
    },

    #[serde(rename = "image_generation")]
    #[non_exhaustive]
    ImageGeneration {
        #[serde(flatten)]
        data: ImageGenerationTool,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ChatCompletionRequestToolsItem {
    pub fn connector(data: CustomConnector) -> Self {
        Self::Connector { data }
    }

    pub fn document_library(data: DocumentLibraryTool) -> Self {
        Self::DocumentLibrary { data }
    }

    pub fn function(data: Tool) -> Self {
        Self::Function { data }
    }

    pub fn image_generation(data: ImageGenerationTool) -> Self {
        Self::ImageGeneration { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
