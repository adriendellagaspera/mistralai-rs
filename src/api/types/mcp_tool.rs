pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpTool {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<McpToolMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ToolExecution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<McpServerIcon>>,
    #[serde(rename = "inputSchema")]
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpTool {
    pub fn builder() -> McpToolBuilder {
        <McpToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolBuilder {
    meta: Option<McpToolMeta>,
    annotations: Option<ToolAnnotations>,
    description: Option<String>,
    execution: Option<ToolExecution>,
    icons: Option<Vec<McpServerIcon>>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
}

impl McpToolBuilder {
    pub fn meta(mut self, value: McpToolMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn annotations(mut self, value: ToolAnnotations) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn execution(mut self, value: ToolExecution) -> Self {
        self.execution = Some(value);
        self
    }

    pub fn icons(mut self, value: Vec<McpServerIcon>) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_schema`](McpToolBuilder::input_schema)
    /// - [`name`](McpToolBuilder::name)
    pub fn build(self) -> Result<McpTool, BuildError> {
        Ok(McpTool {
            meta: self.meta,
            annotations: self.annotations,
            description: self.description,
            execution: self.execution,
            icons: self.icons,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            output_schema: self.output_schema,
            title: self.title,
            extra: Default::default(),
        })
    }
}
