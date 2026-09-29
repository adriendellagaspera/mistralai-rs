pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageInfo {
    #[serde(default)]
    pub prompt_tokens: i64,
    #[serde(default)]
    pub completion_tokens: i64,
    #[serde(default)]
    pub total_tokens: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_audio_seconds: Option<i64>,
    /// The service tier at which the request was processed: standard or priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

impl UsageInfo {
    pub fn builder() -> UsageInfoBuilder {
        <UsageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageInfoBuilder {
    prompt_tokens: Option<i64>,
    completion_tokens: Option<i64>,
    total_tokens: Option<i64>,
    prompt_audio_seconds: Option<i64>,
    service_tier: Option<String>,
}

impl UsageInfoBuilder {
    pub fn prompt_tokens(mut self, value: i64) -> Self {
        self.prompt_tokens = Some(value);
        self
    }

    pub fn completion_tokens(mut self, value: i64) -> Self {
        self.completion_tokens = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    pub fn prompt_audio_seconds(mut self, value: i64) -> Self {
        self.prompt_audio_seconds = Some(value);
        self
    }

    pub fn service_tier(mut self, value: impl Into<String>) -> Self {
        self.service_tier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_tokens`](UsageInfoBuilder::prompt_tokens)
    /// - [`completion_tokens`](UsageInfoBuilder::completion_tokens)
    /// - [`total_tokens`](UsageInfoBuilder::total_tokens)
    pub fn build(self) -> Result<UsageInfo, BuildError> {
        Ok(UsageInfo {
            prompt_tokens: self
                .prompt_tokens
                .ok_or_else(|| BuildError::missing_field("prompt_tokens"))?,
            completion_tokens: self
                .completion_tokens
                .ok_or_else(|| BuildError::missing_field("completion_tokens"))?,
            total_tokens: self
                .total_tokens
                .ok_or_else(|| BuildError::missing_field("total_tokens"))?,
            prompt_audio_seconds: self.prompt_audio_seconds,
            service_tier: self.service_tier,
        })
    }
}
