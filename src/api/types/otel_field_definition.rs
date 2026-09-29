pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OtelFieldDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_aggregations: Option<Vec<MetricAggregation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operators: Option<Vec<OtelFieldDefinitionSupportedOperatorsItem>>,
    pub r#type: OtelFieldDefinitionType,
}

impl OtelFieldDefinition {
    pub fn builder() -> OtelFieldDefinitionBuilder {
        <OtelFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OtelFieldDefinitionBuilder {
    group: Option<String>,
    label: Option<String>,
    name: Option<String>,
    supported_aggregations: Option<Vec<MetricAggregation>>,
    supported_operators: Option<Vec<OtelFieldDefinitionSupportedOperatorsItem>>,
    r#type: Option<OtelFieldDefinitionType>,
}

impl OtelFieldDefinitionBuilder {
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

    pub fn supported_aggregations(mut self, value: Vec<MetricAggregation>) -> Self {
        self.supported_aggregations = Some(value);
        self
    }

    pub fn supported_operators(
        mut self,
        value: Vec<OtelFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        self.supported_operators = Some(value);
        self
    }

    pub fn r#type(mut self, value: OtelFieldDefinitionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OtelFieldDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](OtelFieldDefinitionBuilder::label)
    /// - [`name`](OtelFieldDefinitionBuilder::name)
    /// - [`r#type`](OtelFieldDefinitionBuilder::r#type)
    pub fn build(self) -> Result<OtelFieldDefinition, BuildError> {
        Ok(OtelFieldDefinition {
            group: self.group,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            supported_aggregations: self.supported_aggregations,
            supported_operators: self.supported_operators,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
