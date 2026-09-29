pub use crate::prelude::*;

/// Typed _meta for MCP tools.
///
/// Only the 'ui' field is typed. Other fields are allowed via extra="allow".
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpToolMeta {
    #[serde(rename = "ai.mistral/turbine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mistral_turbine: Option<TurbineToolMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<McpuiToolMeta>,
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
    ai_mistral_turbine: Option<TurbineToolMeta>,
    ui: Option<McpuiToolMeta>,
}

impl McpToolMetaBuilder {
    pub fn ai_mistral_turbine(mut self, value: TurbineToolMeta) -> Self {
        self.ai_mistral_turbine = Some(value);
        self
    }

    pub fn ui(mut self, value: McpuiToolMeta) -> Self {
        self.ui = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolMeta`].
    pub fn build(self) -> Result<McpToolMeta, BuildError> {
        Ok(McpToolMeta {
            ai_mistral_turbine: self.ai_mistral_turbine,
            ui: self.ui,
            extra: Default::default(),
        })
    }
}
