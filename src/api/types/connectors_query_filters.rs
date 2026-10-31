pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorsQueryFilters {
    /// Filter connectors by whether they expose an MCP-compatible tool interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_mcp: Option<bool>,
}

impl ConnectorsQueryFilters {
    pub fn builder() -> ConnectorsQueryFiltersBuilder {
        <ConnectorsQueryFiltersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorsQueryFiltersBuilder {
    supports_mcp: Option<bool>,
}

impl ConnectorsQueryFiltersBuilder {
    pub fn supports_mcp(mut self, value: bool) -> Self {
        self.supports_mcp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorsQueryFilters`].
    pub fn build(self) -> Result<ConnectorsQueryFilters, BuildError> {
        Ok(ConnectorsQueryFilters {
            supports_mcp: self.supports_mcp,
        })
    }
}
