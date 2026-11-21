pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTriggerAction {
    /// ID of the target primitive (workflow, agent, or app)
    #[serde(default)]
    pub primitive_id: String,
    /// Action to execute
    pub r#type: TriggerActionType,
}

impl CreateTriggerAction {
    pub fn builder() -> CreateTriggerActionBuilder {
        <CreateTriggerActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTriggerActionBuilder {
    primitive_id: Option<String>,
    r#type: Option<TriggerActionType>,
}

impl CreateTriggerActionBuilder {
    pub fn primitive_id(mut self, value: impl Into<String>) -> Self {
        self.primitive_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TriggerActionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTriggerAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`primitive_id`](CreateTriggerActionBuilder::primitive_id)
    /// - [`r#type`](CreateTriggerActionBuilder::r#type)
    pub fn build(self) -> Result<CreateTriggerAction, BuildError> {
        Ok(CreateTriggerAction {
            primitive_id: self
                .primitive_id
                .ok_or_else(|| BuildError::missing_field("primitive_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
