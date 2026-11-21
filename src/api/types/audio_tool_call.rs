pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioToolCall {
    /// JSON text, or null when unavailable or withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl AudioToolCall {
    pub fn builder() -> AudioToolCallBuilder {
        <AudioToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioToolCallBuilder {
    arguments: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl AudioToolCallBuilder {
    pub fn arguments(mut self, value: impl Into<String>) -> Self {
        self.arguments = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AudioToolCallBuilder::id)
    /// - [`name`](AudioToolCallBuilder::name)
    pub fn build(self) -> Result<AudioToolCall, BuildError> {
        Ok(AudioToolCall {
            arguments: self.arguments,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
