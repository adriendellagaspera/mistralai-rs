pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechStreamDone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SpeechStreamDoneType>,
    #[serde(default)]
    pub usage: UsageInfo,
}

impl SpeechStreamDone {
    pub fn builder() -> SpeechStreamDoneBuilder {
        <SpeechStreamDoneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechStreamDoneBuilder {
    r#type: Option<SpeechStreamDoneType>,
    usage: Option<UsageInfo>,
}

impl SpeechStreamDoneBuilder {
    pub fn r#type(mut self, value: SpeechStreamDoneType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn usage(mut self, value: UsageInfo) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechStreamDone`].
    /// This method will fail if any of the following fields are not set:
    /// - [`usage`](SpeechStreamDoneBuilder::usage)
    pub fn build(self) -> Result<SpeechStreamDone, BuildError> {
        Ok(SpeechStreamDone {
            r#type: self.r#type,
            usage: self
                .usage
                .ok_or_else(|| BuildError::missing_field("usage"))?,
        })
    }
}
