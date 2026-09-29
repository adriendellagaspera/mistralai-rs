pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Function {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

impl Function {
    pub fn builder() -> FunctionBuilder {
        <FunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionBuilder {
    name: Option<String>,
    description: Option<String>,
    strict: Option<bool>,
    parameters: Option<HashMap<String, serde_json::Value>>,
}

impl FunctionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = Some(value);
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Function`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FunctionBuilder::name)
    /// - [`parameters`](FunctionBuilder::parameters)
    pub fn build(self) -> Result<Function, BuildError> {
        Ok(Function {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            strict: self.strict,
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
