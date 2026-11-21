pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AudioSessionCreateRequestToolsItem {
    #[serde(rename = "connector")]
    #[non_exhaustive]
    Connector {
        #[serde(flatten)]
        data: VoiceAgentCustomConnector,
    },

    #[serde(rename = "document_library")]
    #[non_exhaustive]
    DocumentLibrary {
        #[serde(flatten)]
        data: VoiceAgentDocumentLibraryTool,
    },

    #[serde(rename = "function")]
    #[non_exhaustive]
    Function {
        #[serde(flatten)]
        data: VoiceAgentFunctionToolRequest,
    },

    #[serde(rename = "hang_up")]
    #[non_exhaustive]
    HangUp {
        #[serde(flatten)]
        data: HangUpTool,
    },

    #[serde(rename = "transfer_call")]
    #[non_exhaustive]
    TransferCall {
        #[serde(flatten)]
        data: TransferCallTool,
    },

    #[serde(rename = "web_search")]
    #[non_exhaustive]
    WebSearch {
        #[serde(flatten)]
        data: VoiceAgentWebSearchTool,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AudioSessionCreateRequestToolsItem {
    pub fn connector(data: VoiceAgentCustomConnector) -> Self {
        Self::Connector { data }
    }

    pub fn document_library(data: VoiceAgentDocumentLibraryTool) -> Self {
        Self::DocumentLibrary { data }
    }

    pub fn function(data: VoiceAgentFunctionToolRequest) -> Self {
        Self::Function { data }
    }

    pub fn hang_up(data: HangUpTool) -> Self {
        Self::HangUp { data }
    }

    pub fn transfer_call(data: TransferCallTool) -> Self {
        Self::TransferCall { data }
    }

    pub fn web_search(data: VoiceAgentWebSearchTool) -> Self {
        Self::WebSearch { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
