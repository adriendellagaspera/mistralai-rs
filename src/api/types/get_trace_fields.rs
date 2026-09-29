pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTraceFields {
    #[serde(default)]
    pub field_definitions: Vec<OtelFieldDefinition>,
}

impl GetTraceFields {
    pub fn builder() -> GetTraceFieldsBuilder {
        <GetTraceFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTraceFieldsBuilder {
    field_definitions: Option<Vec<OtelFieldDefinition>>,
}

impl GetTraceFieldsBuilder {
    pub fn field_definitions(mut self, value: Vec<OtelFieldDefinition>) -> Self {
        self.field_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTraceFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](GetTraceFieldsBuilder::field_definitions)
    pub fn build(self) -> Result<GetTraceFields, BuildError> {
        Ok(GetTraceFields {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
        })
    }
}
