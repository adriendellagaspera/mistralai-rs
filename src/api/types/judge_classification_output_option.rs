pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JudgeClassificationOutputOption {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub description: String,
}

impl JudgeClassificationOutputOption {
    pub fn builder() -> JudgeClassificationOutputOptionBuilder {
        <JudgeClassificationOutputOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeClassificationOutputOptionBuilder {
    value: Option<String>,
    description: Option<String>,
}

impl JudgeClassificationOutputOptionBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JudgeClassificationOutputOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](JudgeClassificationOutputOptionBuilder::value)
    /// - [`description`](JudgeClassificationOutputOptionBuilder::description)
    pub fn build(self) -> Result<JudgeClassificationOutputOption, BuildError> {
        Ok(JudgeClassificationOutputOption {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
