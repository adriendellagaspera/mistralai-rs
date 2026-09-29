pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BaseFieldDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    pub r#type: BaseFieldDefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operators: Option<Vec<BaseFieldDefinitionSupportedOperatorsItem>>,
}

impl BaseFieldDefinition {
    pub fn builder() -> BaseFieldDefinitionBuilder {
        <BaseFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BaseFieldDefinitionBuilder {
    name: Option<String>,
    label: Option<String>,
    r#type: Option<BaseFieldDefinitionType>,
    group: Option<String>,
    supported_operators: Option<Vec<BaseFieldDefinitionSupportedOperatorsItem>>,
}

impl BaseFieldDefinitionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: BaseFieldDefinitionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn supported_operators(
        mut self,
        value: Vec<BaseFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        self.supported_operators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BaseFieldDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BaseFieldDefinitionBuilder::name)
    /// - [`label`](BaseFieldDefinitionBuilder::label)
    /// - [`r#type`](BaseFieldDefinitionBuilder::r#type)
    pub fn build(self) -> Result<BaseFieldDefinition, BuildError> {
        Ok(BaseFieldDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            group: self.group,
            supported_operators: self.supported_operators,
        })
    }
}
