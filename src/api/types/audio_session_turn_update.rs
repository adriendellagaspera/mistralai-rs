pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSessionTurnUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interruptions: Option<bool>,
}

impl AudioSessionTurnUpdate {
    pub fn builder() -> AudioSessionTurnUpdateBuilder {
        <AudioSessionTurnUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionTurnUpdateBuilder {
    allow_interruptions: Option<bool>,
}

impl AudioSessionTurnUpdateBuilder {
    pub fn allow_interruptions(mut self, value: bool) -> Self {
        self.allow_interruptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionTurnUpdate`].
    pub fn build(self) -> Result<AudioSessionTurnUpdate, BuildError> {
        Ok(AudioSessionTurnUpdate {
            allow_interruptions: self.allow_interruptions,
        })
    }
}
