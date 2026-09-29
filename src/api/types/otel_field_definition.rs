pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OtelFieldDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    pub r#type: OtelFieldDefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operators: Option<Vec<OtelFieldDefinitionSupportedOperatorsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_aggregations: Option<Vec<MetricAggregation>>,
}

impl OtelFieldDefinition {
    pub fn builder() -> OtelFieldDefinitionBuilder {
        <OtelFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OtelFieldDefinitionBuilder {
    name: Option<String>,
    label: Option<String>,
    r#type: Option<OtelFieldDefinitionType>,
    group: Option<String>,
    supported_operators: Option<Vec<OtelFieldDefinitionSupportedOperatorsItem>>,
    supported_aggregations: Option<Vec<MetricAggregation>>,
}

impl OtelFieldDefinitionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: OtelFieldDefinitionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn supported_operators(
        mut self,
        value: Vec<OtelFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        self.supported_operators = Some(value);
        self
    }

    pub fn supported_aggregations(mut self, value: Vec<MetricAggregation>) -> Self {
        self.supported_aggregations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OtelFieldDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OtelFieldDefinitionBuilder::name)
    /// - [`label`](OtelFieldDefinitionBuilder::label)
    /// - [`r#type`](OtelFieldDefinitionBuilder::r#type)
    pub fn build(self) -> Result<OtelFieldDefinition, BuildError> {
        Ok(OtelFieldDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            group: self.group,
            supported_operators: self.supported_operators,
            supported_aggregations: self.supported_aggregations,
        })
    }
}
