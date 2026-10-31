pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CreateResponseResponseToolsItem {
    #[serde(rename = "function")]
    #[non_exhaustive]
    Function {
        #[serde(flatten)]
        data: ResponsesFunctionTool,
    },

    #[serde(rename = "namespace")]
    #[non_exhaustive]
    Namespace {
        #[serde(flatten)]
        data: ResponsesNamespaceTool,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateResponseResponseToolsItem {
    pub fn function(data: ResponsesFunctionTool) -> Self {
        Self::Function { data }
    }

    pub fn namespace(data: ResponsesNamespaceTool) -> Self {
        Self::Namespace { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
