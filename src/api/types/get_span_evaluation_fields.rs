pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanEvaluationFields {
    #[serde(default)]
    pub field_definitions: Vec<OtelFieldDefinition>,
}

impl GetSpanEvaluationFields {
    pub fn builder() -> GetSpanEvaluationFieldsBuilder {
        <GetSpanEvaluationFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanEvaluationFieldsBuilder {
    field_definitions: Option<Vec<OtelFieldDefinition>>,
}

impl GetSpanEvaluationFieldsBuilder {
    pub fn field_definitions(mut self, value: Vec<OtelFieldDefinition>) -> Self {
        self.field_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanEvaluationFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](GetSpanEvaluationFieldsBuilder::field_definitions)
    pub fn build(self) -> Result<GetSpanEvaluationFields, BuildError> {
        Ok(GetSpanEvaluationFields {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
        })
    }
}
