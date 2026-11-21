pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetPipelineResultFieldsFieldDefinitionsItem {
    #[serde(rename = "ENUM")]
    #[non_exhaustive]
    Enum {
        #[serde(skip_serializing_if = "Option::is_none")]
        constraints: Option<EnumConstraints>,
        #[serde(skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        #[serde(default)]
        label: String,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        source_attribute_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supported_aggregations: Option<Vec<MetricAggregation>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supported_operators: Option<Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>>,
    },

    #[serde(rename = "FLOAT")]
    #[non_exhaustive]
    Float {
        #[serde(skip_serializing_if = "Option::is_none")]
        constraints: Option<FloatConstraints>,
        #[serde(skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        #[serde(default)]
        label: String,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        source_attribute_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supported_aggregations: Option<Vec<MetricAggregation>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supported_operators: Option<Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl GetPipelineResultFieldsFieldDefinitionsItem {
    pub fn r#enum(label: String, name: String) -> Self {
        Self::Enum {
            constraints: None,
            group: None,
            label,
            name,
            source_attribute_key: None,
            supported_aggregations: None,
            supported_operators: None,
        }
    }

    pub fn float(label: String, name: String) -> Self {
        Self::Float {
            constraints: None,
            group: None,
            label,
            name,
            source_attribute_key: None,
            supported_aggregations: None,
            supported_operators: None,
        }
    }

    pub fn enum_with_constraints(
        constraints: EnumConstraints,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Enum {
            constraints: Some(constraints),
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn enum_with_group(
        constraints: Option<EnumConstraints>,
        group: String,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Enum {
            constraints,
            group: Some(group),
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn enum_with_source_attribute_key(
        constraints: Option<EnumConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: String,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Enum {
            constraints,
            group,
            label,
            name,
            source_attribute_key: Some(source_attribute_key),
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn enum_with_supported_aggregations(
        constraints: Option<EnumConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Vec<MetricAggregation>,
        supported_operators: Option<Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Enum {
            constraints,
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations: Some(supported_aggregations),
            supported_operators,
        }
    }

    pub fn enum_with_supported_operators(
        constraints: Option<EnumConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Vec<OtelEnumFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        Self::Enum {
            constraints,
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators: Some(supported_operators),
        }
    }

    pub fn float_with_constraints(
        constraints: FloatConstraints,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Float {
            constraints: Some(constraints),
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn float_with_group(
        constraints: Option<FloatConstraints>,
        group: String,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Float {
            constraints,
            group: Some(group),
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn float_with_source_attribute_key(
        constraints: Option<FloatConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: String,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Option<Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Float {
            constraints,
            group,
            label,
            name,
            source_attribute_key: Some(source_attribute_key),
            supported_aggregations,
            supported_operators,
        }
    }

    pub fn float_with_supported_aggregations(
        constraints: Option<FloatConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Vec<MetricAggregation>,
        supported_operators: Option<Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>>,
    ) -> Self {
        Self::Float {
            constraints,
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations: Some(supported_aggregations),
            supported_operators,
        }
    }

    pub fn float_with_supported_operators(
        constraints: Option<FloatConstraints>,
        group: Option<String>,
        label: String,
        name: String,
        source_attribute_key: Option<String>,
        supported_aggregations: Option<Vec<MetricAggregation>>,
        supported_operators: Vec<OtelFloatFieldDefinitionSupportedOperatorsItem>,
    ) -> Self {
        Self::Float {
            constraints,
            group,
            label,
            name,
            source_attribute_key,
            supported_aggregations,
            supported_operators: Some(supported_operators),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
