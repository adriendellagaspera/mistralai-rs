pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TriggerAction {
    #[serde(default)]
    pub primitive_id: String,
    pub r#type: TriggerActionType,
}

impl TriggerAction {
    pub fn builder() -> TriggerActionBuilder {
        <TriggerActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerActionBuilder {
    primitive_id: Option<String>,
    r#type: Option<TriggerActionType>,
}

impl TriggerActionBuilder {
    pub fn primitive_id(mut self, value: impl Into<String>) -> Self {
        self.primitive_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TriggerActionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TriggerAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`primitive_id`](TriggerActionBuilder::primitive_id)
    /// - [`r#type`](TriggerActionBuilder::r#type)
    pub fn build(self) -> Result<TriggerAction, BuildError> {
        Ok(TriggerAction {
            primitive_id: self
                .primitive_id
                .ok_or_else(|| BuildError::missing_field("primitive_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
