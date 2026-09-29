pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLogFields {
    #[serde(default)]
    pub field_definitions: Vec<OtelFieldDefinition>,
}

impl GetLogFields {
    pub fn builder() -> GetLogFieldsBuilder {
        <GetLogFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLogFieldsBuilder {
    field_definitions: Option<Vec<OtelFieldDefinition>>,
}

impl GetLogFieldsBuilder {
    pub fn field_definitions(mut self, value: Vec<OtelFieldDefinition>) -> Self {
        self.field_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLogFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](GetLogFieldsBuilder::field_definitions)
    pub fn build(self) -> Result<GetLogFields, BuildError> {
        Ok(GetLogFields {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
        })
    }
}
