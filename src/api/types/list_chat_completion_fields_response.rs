pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChatCompletionFieldsResponse {
    #[serde(default)]
    pub field_definitions: Vec<BaseFieldDefinition>,
    #[serde(default)]
    pub field_groups: Vec<FieldGroup>,
}

impl ListChatCompletionFieldsResponse {
    pub fn builder() -> ListChatCompletionFieldsResponseBuilder {
        <ListChatCompletionFieldsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChatCompletionFieldsResponseBuilder {
    field_definitions: Option<Vec<BaseFieldDefinition>>,
    field_groups: Option<Vec<FieldGroup>>,
}

impl ListChatCompletionFieldsResponseBuilder {
    pub fn field_definitions(mut self, value: Vec<BaseFieldDefinition>) -> Self {
        self.field_definitions = Some(value);
        self
    }

    pub fn field_groups(mut self, value: Vec<FieldGroup>) -> Self {
        self.field_groups = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChatCompletionFieldsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_definitions`](ListChatCompletionFieldsResponseBuilder::field_definitions)
    /// - [`field_groups`](ListChatCompletionFieldsResponseBuilder::field_groups)
    pub fn build(self) -> Result<ListChatCompletionFieldsResponse, BuildError> {
        Ok(ListChatCompletionFieldsResponse {
            field_definitions: self
                .field_definitions
                .ok_or_else(|| BuildError::missing_field("field_definitions"))?,
            field_groups: self
                .field_groups
                .ok_or_else(|| BuildError::missing_field("field_groups"))?,
        })
    }
}
