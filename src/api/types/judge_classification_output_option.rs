pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JudgeClassificationOutputOption {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub value: String,
}

impl JudgeClassificationOutputOption {
    pub fn builder() -> JudgeClassificationOutputOptionBuilder {
        <JudgeClassificationOutputOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeClassificationOutputOptionBuilder {
    description: Option<String>,
    value: Option<String>,
}

impl JudgeClassificationOutputOptionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JudgeClassificationOutputOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](JudgeClassificationOutputOptionBuilder::description)
    /// - [`value`](JudgeClassificationOutputOptionBuilder::value)
    pub fn build(self) -> Result<JudgeClassificationOutputOption, BuildError> {
        Ok(JudgeClassificationOutputOption {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
