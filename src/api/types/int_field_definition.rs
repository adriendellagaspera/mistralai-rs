pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IntFieldDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

impl IntFieldDefinition {
    pub fn builder() -> IntFieldDefinitionBuilder {
        <IntFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntFieldDefinitionBuilder {
    default: Option<serde_json::Value>,
    indexed: Option<bool>,
    required: Option<bool>,
    system: Option<bool>,
}

impl IntFieldDefinitionBuilder {
    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn indexed(mut self, value: bool) -> Self {
        self.indexed = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn system(mut self, value: bool) -> Self {
        self.system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntFieldDefinition`].
    pub fn build(self) -> Result<IntFieldDefinition, BuildError> {
        Ok(IntFieldDefinition {
            default: self.default,
            indexed: self.indexed,
            required: self.required,
            system: self.system,
        })
    }
}
