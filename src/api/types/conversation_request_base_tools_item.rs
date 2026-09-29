pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationRequestBaseToolsItem {
    #[serde(rename = "code_interpreter")]
    #[non_exhaustive]
    CodeInterpreter {
        #[serde(flatten)]
        data: CodeInterpreterTool,
    },

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
        data: FunctionTool,
    },

    #[serde(rename = "image_generation")]
    #[non_exhaustive]
    ImageGeneration {
        #[serde(flatten)]
        data: ImageGenerationTool,
    },

    #[serde(rename = "web_search")]
    #[non_exhaustive]
    WebSearch {
        #[serde(flatten)]
        data: WebSearchTool,
    },

    #[serde(rename = "web_search_premium")]
    #[non_exhaustive]
    WebSearchPremium {
        #[serde(flatten)]
        data: WebSearchPremiumTool,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConversationRequestBaseToolsItem {
    pub fn code_interpreter(data: CodeInterpreterTool) -> Self {
        Self::CodeInterpreter { data }
    }

    pub fn connector(data: CustomConnector) -> Self {
        Self::Connector { data }
    }

    pub fn document_library(data: DocumentLibraryTool) -> Self {
        Self::DocumentLibrary { data }
    }

    pub fn function(data: FunctionTool) -> Self {
        Self::Function { data }
    }

    pub fn image_generation(data: ImageGenerationTool) -> Self {
        Self::ImageGeneration { data }
    }

    pub fn web_search(data: WebSearchTool) -> Self {
        Self::WebSearch { data }
    }

    pub fn web_search_premium(data: WebSearchPremiumTool) -> Self {
        Self::WebSearchPremium { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
