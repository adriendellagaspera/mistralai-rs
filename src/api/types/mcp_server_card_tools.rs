pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum McpServerCardTools {
    McpServerCardToolsZero(McpServerCardToolsZero),

    McpServerCardToolList(Vec<McpServerCardTool>),
}

impl McpServerCardTools {
    pub fn is_mcp_server_card_tools_zero(&self) -> bool {
        matches!(self, Self::McpServerCardToolsZero(_))
    }

    pub fn is_mcp_server_card_tool_list(&self) -> bool {
        matches!(self, Self::McpServerCardToolList(_))
    }

    pub fn as_mcp_server_card_tools_zero(&self) -> Option<&McpServerCardToolsZero> {
        match self {
            Self::McpServerCardToolsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_server_card_tools_zero(self) -> Option<McpServerCardToolsZero> {
        match self {
            Self::McpServerCardToolsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_mcp_server_card_tool_list(&self) -> Option<&Vec<McpServerCardTool>> {
        match self {
            Self::McpServerCardToolList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_server_card_tool_list(self) -> Option<Vec<McpServerCardTool>> {
        match self {
            Self::McpServerCardToolList(value) => Some(value),
            _ => None,
        }
    }
}
