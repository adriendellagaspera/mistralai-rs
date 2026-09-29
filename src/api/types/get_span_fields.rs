pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanFields {
    #[serde(default)]
    pub field_definitions: Vec<OtelFieldDefinition>,
}

impl GetSpanFields {
    pub fn builder() -> GetSpanFieldsBuilder {
        <GetSpanFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanFieldsBuilder {
    field_definitions: Option<Vec<OtelFieldDefinition>>,
}

impl GetSpanFieldsBuilder {
    pub fn field_definitions(mut self, value: Vec<OtelFieldDefinition>) -> Self {
        self.field_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](GetSpanFieldsBuilder::field_definitions)
    pub fn build(self) -> Result<GetSpanFields, BuildError> {
        Ok(GetSpanFields {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
        })
    }
}
