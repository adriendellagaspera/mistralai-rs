pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptVariable {
    /// Stable object name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PromptVariable {
    pub fn builder() -> PromptVariableBuilder {
        <PromptVariableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptVariableBuilder {
    name: Option<String>,
}

impl PromptVariableBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromptVariable`].
    pub fn build(self) -> Result<PromptVariable, BuildError> {
        Ok(PromptVariable { name: self.name })
    }
}
