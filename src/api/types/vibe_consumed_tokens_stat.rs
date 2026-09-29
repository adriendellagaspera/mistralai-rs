pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeConsumedTokensStat {
    #[serde(default)]
    pub cached_tokens: i64,
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub output_tokens: i64,
}

impl VibeConsumedTokensStat {
    pub fn builder() -> VibeConsumedTokensStatBuilder {
        <VibeConsumedTokensStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeConsumedTokensStatBuilder {
    cached_tokens: Option<i64>,
    day: Option<NaiveDate>,
    input_tokens: Option<i64>,
    model: Option<String>,
    output_tokens: Option<i64>,
}

impl VibeConsumedTokensStatBuilder {
    pub fn cached_tokens(mut self, value: i64) -> Self {
        self.cached_tokens = Some(value);
        self
    }

    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeConsumedTokensStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cached_tokens`](VibeConsumedTokensStatBuilder::cached_tokens)
    /// - [`day`](VibeConsumedTokensStatBuilder::day)
    /// - [`input_tokens`](VibeConsumedTokensStatBuilder::input_tokens)
    /// - [`model`](VibeConsumedTokensStatBuilder::model)
    /// - [`output_tokens`](VibeConsumedTokensStatBuilder::output_tokens)
    pub fn build(self) -> Result<VibeConsumedTokensStat, BuildError> {
        Ok(VibeConsumedTokensStat {
            cached_tokens: self
                .cached_tokens
                .ok_or_else(|| BuildError::missing_field("cached_tokens"))?,
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            input_tokens: self
                .input_tokens
                .ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            output_tokens: self
                .output_tokens
                .ok_or_else(|| BuildError::missing_field("output_tokens"))?,
        })
    }
}
