pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Function {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl Function {
    pub fn builder() -> FunctionBuilder {
        <FunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionBuilder {
    description: Option<String>,
    name: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
    strict: Option<bool>,
}

impl FunctionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`Function`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FunctionBuilder::name)
    /// - [`parameters`](FunctionBuilder::parameters)
    pub fn build(self) -> Result<Function, BuildError> {
        Ok(Function {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
            strict: self.strict,
        })
    }
}
