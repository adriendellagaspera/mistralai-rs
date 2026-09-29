pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum McpServerCardPrompts {
    McpServerCardPromptsZero(McpServerCardPromptsZero),

    McpPromptList(Vec<McpPrompt>),
}

impl McpServerCardPrompts {
    pub fn is_mcp_server_card_prompts_zero(&self) -> bool {
        matches!(self, Self::McpServerCardPromptsZero(_))
    }

    pub fn is_mcp_prompt_list(&self) -> bool {
        matches!(self, Self::McpPromptList(_))
    }

    pub fn as_mcp_server_card_prompts_zero(&self) -> Option<&McpServerCardPromptsZero> {
        match self {
            Self::McpServerCardPromptsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_server_card_prompts_zero(self) -> Option<McpServerCardPromptsZero> {
        match self {
            Self::McpServerCardPromptsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_mcp_prompt_list(&self) -> Option<&Vec<McpPrompt>> {
        match self {
            Self::McpPromptList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_mcp_prompt_list(self) -> Option<Vec<McpPrompt>> {
        match self {
            Self::McpPromptList(value) => Some(value),
            _ => None,
        }
    }
}
