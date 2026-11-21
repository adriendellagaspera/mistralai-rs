pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPipelineResultFields {
    #[serde(default)]
    pub field_definitions: Vec<GetPipelineResultFieldsFieldDefinitionsItem>,
}

impl GetPipelineResultFields {
    pub fn builder() -> GetPipelineResultFieldsBuilder {
        <GetPipelineResultFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPipelineResultFieldsBuilder {
    field_definitions: Option<Vec<GetPipelineResultFieldsFieldDefinitionsItem>>,
}

impl GetPipelineResultFieldsBuilder {
    pub fn field_definitions(
        mut self,
        value: Vec<GetPipelineResultFieldsFieldDefinitionsItem>,
    ) -> Self {
        self.field_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPipelineResultFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](GetPipelineResultFieldsBuilder::field_definitions)
    pub fn build(self) -> Result<GetPipelineResultFields, BuildError> {
        Ok(GetPipelineResultFields {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
        })
    }
}
