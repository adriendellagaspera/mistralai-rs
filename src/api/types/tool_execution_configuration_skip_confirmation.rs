pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolExecutionConfigurationSkipConfirmation {
    StringList(Vec<String>),

    LogicalExpression(LogicalExpression),

    ToolProperties(ToolProperties),
}

impl ToolExecutionConfigurationSkipConfirmation {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_logical_expression(&self) -> bool {
        matches!(self, Self::LogicalExpression(_))
    }

    pub fn is_tool_properties(&self) -> bool {
        matches!(self, Self::ToolProperties(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_logical_expression(&self) -> Option<&LogicalExpression> {
        match self {
            Self::LogicalExpression(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_logical_expression(self) -> Option<LogicalExpression> {
        match self {
            Self::LogicalExpression(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tool_properties(&self) -> Option<&ToolProperties> {
        match self {
            Self::ToolProperties(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tool_properties(self) -> Option<ToolProperties> {
        match self {
            Self::ToolProperties(value) => Some(value),
            _ => None,
        }
    }
}
