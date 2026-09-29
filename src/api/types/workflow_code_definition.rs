pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowCodeDefinition {
    /// Whether the workflow enforces deterministic execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_determinism: Option<bool>,
    /// Maximum total execution time including retries and continue-as-new
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub execution_timeout: Option<f64>,
    /// Input schema of the workflow's run method
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    /// Whether the workflow must run associated to a user's identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<bool>,
    /// Output schema of the workflow's run method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
    /// Plugin-specific metadata (e.g. connector declarations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Query handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<QueryDefinition>>,
    /// Signal handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalDefinition>>,
    /// Update handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<UpdateDefinition>>,
}

impl WorkflowCodeDefinition {
    pub fn builder() -> WorkflowCodeDefinitionBuilder {
        <WorkflowCodeDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowCodeDefinitionBuilder {
    enforce_determinism: Option<bool>,
    execution_timeout: Option<f64>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    on_behalf_of: Option<bool>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
    plugin_metadata: Option<HashMap<String, serde_json::Value>>,
    queries: Option<Vec<QueryDefinition>>,
    signals: Option<Vec<SignalDefinition>>,
    updates: Option<Vec<UpdateDefinition>>,
}

impl WorkflowCodeDefinitionBuilder {
    pub fn enforce_determinism(mut self, value: bool) -> Self {
        self.enforce_determinism = Some(value);
        self
    }

    pub fn execution_timeout(mut self, value: f64) -> Self {
        self.execution_timeout = Some(value);
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn on_behalf_of(mut self, value: bool) -> Self {
        self.on_behalf_of = Some(value);
        self
    }

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn plugin_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.plugin_metadata = Some(value);
        self
    }

    pub fn queries(mut self, value: Vec<QueryDefinition>) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn signals(mut self, value: Vec<SignalDefinition>) -> Self {
        self.signals = Some(value);
        self
    }

    pub fn updates(mut self, value: Vec<UpdateDefinition>) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowCodeDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_schema`](WorkflowCodeDefinitionBuilder::input_schema)
    pub fn build(self) -> Result<WorkflowCodeDefinition, BuildError> {
        Ok(WorkflowCodeDefinition {
            enforce_determinism: self.enforce_determinism,
            execution_timeout: self.execution_timeout,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
            on_behalf_of: self.on_behalf_of,
            output_schema: self.output_schema,
            plugin_metadata: self.plugin_metadata,
            queries: self.queries,
            signals: self.signals,
            updates: self.updates,
        })
    }
}
