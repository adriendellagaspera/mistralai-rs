pub use crate::prelude::*;

/// Typed _meta for MCP tools.
///
/// Only the 'ui' field is typed. Other fields are allowed via extra="allow".
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpToolMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<McpuiToolMeta>,
    #[serde(rename = "ai.mistral/turbine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mistral_turbine: Option<TurbineToolMeta>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpToolMeta {
    pub fn builder() -> McpToolMetaBuilder {
        <McpToolMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolMetaBuilder {
    ui: Option<McpuiToolMeta>,
    ai_mistral_turbine: Option<TurbineToolMeta>,
}

impl McpToolMetaBuilder {
    pub fn ui(mut self, value: McpuiToolMeta) -> Self {
        self.ui = Some(value);
        self
    }

    pub fn ai_mistral_turbine(mut self, value: TurbineToolMeta) -> Self {
        self.ai_mistral_turbine = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolMeta`].
    pub fn build(self) -> Result<McpToolMeta, BuildError> {
        Ok(McpToolMeta {
            ui: self.ui,
            ai_mistral_turbine: self.ai_mistral_turbine,
            extra: Default::default(),
        })
    }
}
