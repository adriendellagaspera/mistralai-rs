pub use crate::prelude::*;

/// Authentication requirements for a remote transport (SEP-2127).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServerAuthenticationRequirement {
    /// Whether authentication is mandatory
    #[serde(default)]
    pub required: bool,
    /// Supported schemes (e.g. ['bearer', 'oauth2'])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<String>>,
}

impl McpServerAuthenticationRequirement {
    pub fn builder() -> McpServerAuthenticationRequirementBuilder {
        <McpServerAuthenticationRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerAuthenticationRequirementBuilder {
    required: Option<bool>,
    schemes: Option<Vec<String>>,
}

impl McpServerAuthenticationRequirementBuilder {
    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn schemes(mut self, value: Vec<String>) -> Self {
        self.schemes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerAuthenticationRequirement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](McpServerAuthenticationRequirementBuilder::required)
    pub fn build(self) -> Result<McpServerAuthenticationRequirement, BuildError> {
        Ok(McpServerAuthenticationRequirement {
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            schemes: self.schemes,
        })
    }
}
