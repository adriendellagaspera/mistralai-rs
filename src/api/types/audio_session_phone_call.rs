pub use crate::prelude::*;

/// The phone line a session is attached to.
///
/// Present only when the caller reached the agent over telephony, `null` for a
/// session started from a browser or the API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSessionPhoneCall {
    /// The number as it stands now, or null once it has been released or when you cannot read it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<AudioSessionPhoneNumber>,
    #[serde(default)]
    pub phone_number_id: String,
}

impl AudioSessionPhoneCall {
    pub fn builder() -> AudioSessionPhoneCallBuilder {
        <AudioSessionPhoneCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionPhoneCallBuilder {
    phone_number: Option<AudioSessionPhoneNumber>,
    phone_number_id: Option<String>,
}

impl AudioSessionPhoneCallBuilder {
    pub fn phone_number(mut self, value: AudioSessionPhoneNumber) -> Self {
        self.phone_number = Some(value);
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionPhoneCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number_id`](AudioSessionPhoneCallBuilder::phone_number_id)
    pub fn build(self) -> Result<AudioSessionPhoneCall, BuildError> {
        Ok(AudioSessionPhoneCall {
            phone_number: self.phone_number,
            phone_number_id: self
                .phone_number_id
                .ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
        })
    }
}
