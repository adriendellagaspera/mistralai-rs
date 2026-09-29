pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConnectorListToolsV1ConnectorsResponse {
    McpToolList(Vec<McpTool>),

    StringToValueMapList(Vec<HashMap<String, serde_json::Value>>),
}

impl ConnectorListToolsV1ConnectorsResponse {
    pub fn is_mcp_tool_list(&self) -> bool {
        matches!(self, Self::McpToolList(_))
    }

    pub fn is_string_to_value_map_list(&self) -> bool {
        matches!(self, Self::StringToValueMapList(_))
    }

    pub fn as_mcp_tool_list(&self) -> Option<&Vec<McpTool>> {
        match self {
            Self::McpToolList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_tool_list(self) -> Option<Vec<McpTool>> {
        match self {
            Self::McpToolList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_to_value_map_list(&self) -> Option<&Vec<HashMap<String, serde_json::Value>>> {
        match self {
            Self::StringToValueMapList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_value_map_list(self) -> Option<Vec<HashMap<String, serde_json::Value>>> {
        match self {
            Self::StringToValueMapList(value) => Some(value),
            _ => None,
        }
    }
}
