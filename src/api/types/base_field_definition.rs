pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BaseFieldDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operators: Option<Vec<BaseFieldDefinitionSupportedOperatorsItem>>,
    pub r#type: BaseFieldDefinitionType,
}

impl BaseFieldDefinition {
    pub fn builder() -> BaseFieldDefinitionBuilder {
        <BaseFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BaseFieldDefinitionBuilder {
    group: Option<String>,
    label: Option<String>,
    name: Option<String>,
    supported_operators: Option<Vec<BaseFieldDefinitionSupportedOperatorsItem>>,
    r#type: Option<BaseFieldDefinitionType>,
}

impl BaseFieldDefinitionBuilder {
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn supported_operators(
        mut self,
        value: Vec<BaseFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        self.supported_operators = Some(value);
        self
    }

    pub fn r#type(mut self, value: BaseFieldDefinitionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BaseFieldDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](BaseFieldDefinitionBuilder::label)
    /// - [`name`](BaseFieldDefinitionBuilder::name)
    /// - [`r#type`](BaseFieldDefinitionBuilder::r#type)
    pub fn build(self) -> Result<BaseFieldDefinition, BuildError> {
        Ok(BaseFieldDefinition {
            group: self.group,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            supported_operators: self.supported_operators,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
