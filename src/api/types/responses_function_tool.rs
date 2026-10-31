pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesFunctionTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<ResponsesFunctionToolAllowedCallersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_loading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFunctionTool {
    pub fn builder() -> ResponsesFunctionToolBuilder {
        <ResponsesFunctionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFunctionToolBuilder {
    allowed_callers: Option<Vec<ResponsesFunctionToolAllowedCallersItem>>,
    defer_loading: Option<bool>,
    description: Option<String>,
    name: Option<String>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
    parameters: Option<HashMap<String, serde_json::Value>>,
    strict: Option<bool>,
}

impl ResponsesFunctionToolBuilder {
    pub fn allowed_callers(mut self, value: Vec<ResponsesFunctionToolAllowedCallersItem>) -> Self {
        self.allowed_callers = Some(value);
        self
    }

    pub fn defer_loading(mut self, value: bool) -> Self {
        self.defer_loading = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFunctionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesFunctionToolBuilder::name)
    pub fn build(self) -> Result<ResponsesFunctionTool, BuildError> {
        Ok(ResponsesFunctionTool {
            allowed_callers: self.allowed_callers,
            defer_loading: self.defer_loading,
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            output_schema: self.output_schema,
            parameters: self.parameters,
            strict: self.strict,
            extra: Default::default(),
        })
    }
}
