#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionFields {
    pub field_definitions: Vec<BaseFieldDefinition>,
    pub field_groups: Vec<FieldGroup>,
}
