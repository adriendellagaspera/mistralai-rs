pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeConsumedTokensStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub cached_tokens: i64,
    #[serde(default)]
    pub input_tokens: i64,
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
    day: Option<NaiveDate>,
    model: Option<String>,
    cached_tokens: Option<i64>,
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}

impl VibeConsumedTokensStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn cached_tokens(mut self, value: i64) -> Self {
        self.cached_tokens = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeConsumedTokensStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeConsumedTokensStatBuilder::day)
    /// - [`model`](VibeConsumedTokensStatBuilder::model)
    /// - [`cached_tokens`](VibeConsumedTokensStatBuilder::cached_tokens)
    /// - [`input_tokens`](VibeConsumedTokensStatBuilder::input_tokens)
    /// - [`output_tokens`](VibeConsumedTokensStatBuilder::output_tokens)
    pub fn build(self) -> Result<VibeConsumedTokensStat, BuildError> {
        Ok(VibeConsumedTokensStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            cached_tokens: self
                .cached_tokens
                .ok_or_else(|| BuildError::missing_field("cached_tokens"))?,
            input_tokens: self
                .input_tokens
                .ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            output_tokens: self
                .output_tokens
                .ok_or_else(|| BuildError::missing_field("output_tokens"))?,
        })
    }
}
