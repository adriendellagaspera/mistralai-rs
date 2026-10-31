pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModerationDefinitionV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_attributes: Option<Vec<String>>,
}

impl ModerationDefinitionV1 {
    pub fn builder() -> ModerationDefinitionV1Builder {
        <ModerationDefinitionV1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationDefinitionV1Builder {
    model: Option<String>,
    target_attributes: Option<Vec<String>>,
}

impl ModerationDefinitionV1Builder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn target_attributes(mut self, value: Vec<String>) -> Self {
        self.target_attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationDefinitionV1`].
    pub fn build(self) -> Result<ModerationDefinitionV1, BuildError> {
        Ok(ModerationDefinitionV1 {
            model: self.model,
            target_attributes: self.target_attributes,
        })
    }
}
