pub use crate::prelude::*;

/// Header definition for a remote transport (SEP-2127).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServerRemoteHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Human-readable description of the header
    #[serde(default)]
    pub description: String,
    #[serde(rename = "isRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "isSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    /// Header name
    #[serde(default)]
    pub name: String,
}

impl McpServerRemoteHeader {
    pub fn builder() -> McpServerRemoteHeaderBuilder {
        <McpServerRemoteHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerRemoteHeaderBuilder {
    choices: Option<Vec<String>>,
    default: Option<String>,
    description: Option<String>,
    is_required: Option<bool>,
    is_secret: Option<bool>,
    name: Option<String>,
}

impl McpServerRemoteHeaderBuilder {
    pub fn choices(mut self, value: Vec<String>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerRemoteHeader`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](McpServerRemoteHeaderBuilder::description)
    /// - [`name`](McpServerRemoteHeaderBuilder::name)
    pub fn build(self) -> Result<McpServerRemoteHeader, BuildError> {
        Ok(McpServerRemoteHeader {
            choices: self.choices,
            default: self.default,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            is_required: self.is_required,
            is_secret: self.is_secret,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
