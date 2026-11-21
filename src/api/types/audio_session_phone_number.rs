pub use crate::prelude::*;

/// The line the call came in on, as it stands now.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSessionPhoneNumber {
    #[serde(default)]
    pub id: String,
    /// What the caller dialed to reach this number.
    #[serde(default)]
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AudioSessionPhoneNumber {
    pub fn builder() -> AudioSessionPhoneNumberBuilder {
        <AudioSessionPhoneNumberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionPhoneNumberBuilder {
    id: Option<String>,
    identifier: Option<String>,
    name: Option<String>,
}

impl AudioSessionPhoneNumberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionPhoneNumber`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AudioSessionPhoneNumberBuilder::id)
    /// - [`identifier`](AudioSessionPhoneNumberBuilder::identifier)
    pub fn build(self) -> Result<AudioSessionPhoneNumber, BuildError> {
        Ok(AudioSessionPhoneNumber {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self
                .identifier
                .ok_or_else(|| BuildError::missing_field("identifier"))?,
            name: self.name,
        })
    }
}
