pub use crate::prelude::*;

/// A filler the model writes for each call, in the user's language.
///
/// `instructions` guide what it says. `fallback` is spoken when it writes none;
/// without one, the call is silent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneratedToolFiller {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<GeneratedToolFillerFallback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

impl GeneratedToolFiller {
    pub fn builder() -> GeneratedToolFillerBuilder {
        <GeneratedToolFillerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratedToolFillerBuilder {
    fallback: Option<GeneratedToolFillerFallback>,
    instructions: Option<String>,
}

impl GeneratedToolFillerBuilder {
    pub fn fallback(mut self, value: GeneratedToolFillerFallback) -> Self {
        self.fallback = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneratedToolFiller`].
    pub fn build(self) -> Result<GeneratedToolFiller, BuildError> {
        Ok(GeneratedToolFiller {
            fallback: self.fallback,
            instructions: self.instructions,
        })
    }
}
