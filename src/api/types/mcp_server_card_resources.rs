pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum McpServerCardResources {
    McpServerCardResourcesZero(McpServerCardResourcesZero),

    McpResourceList(Vec<McpResource>),
}

impl McpServerCardResources {
    pub fn is_mcp_server_card_resources_zero(&self) -> bool {
        matches!(self, Self::McpServerCardResourcesZero(_))
    }

    pub fn is_mcp_resource_list(&self) -> bool {
        matches!(self, Self::McpResourceList(_))
    }

    pub fn as_mcp_server_card_resources_zero(&self) -> Option<&McpServerCardResourcesZero> {
        match self {
            Self::McpServerCardResourcesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_server_card_resources_zero(self) -> Option<McpServerCardResourcesZero> {
        match self {
            Self::McpServerCardResourcesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_mcp_resource_list(&self) -> Option<&Vec<McpResource>> {
        match self {
            Self::McpResourceList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_resource_list(self) -> Option<Vec<McpResource>> {
        match self {
            Self::McpResourceList(value) => Some(value),
            _ => None,
        }
    }
}
