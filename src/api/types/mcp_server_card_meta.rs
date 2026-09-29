pub use crate::prelude::*;

/// Typed _meta for MCP server cards.
///
/// Only the 'turbine' field is typed. Other fields are allowed via extra="allow".
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServerCardMeta {
    #[serde(rename = "ai.mistral/turbine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mistral_turbine: Option<TurbineMeta>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpServerCardMeta {
    pub fn builder() -> McpServerCardMetaBuilder {
        <McpServerCardMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerCardMetaBuilder {
    ai_mistral_turbine: Option<TurbineMeta>,
}

impl McpServerCardMetaBuilder {
    pub fn ai_mistral_turbine(mut self, value: TurbineMeta) -> Self {
        self.ai_mistral_turbine = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerCardMeta`].
    pub fn build(self) -> Result<McpServerCardMeta, BuildError> {
        Ok(McpServerCardMeta {
            ai_mistral_turbine: self.ai_mistral_turbine,
            extra: Default::default(),
        })
    }
}
