pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RawAudio {
    #[serde(default)]
    pub data: String,
}

impl RawAudio {
    pub fn builder() -> RawAudioBuilder {
        <RawAudioBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawAudioBuilder {
    data: Option<String>,
}

impl RawAudioBuilder {
    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RawAudio`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RawAudioBuilder::data)
    pub fn build(self) -> Result<RawAudio, BuildError> {
        Ok(RawAudio {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
