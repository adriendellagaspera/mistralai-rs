pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JudgeClassificationOutput {
    #[serde(default)]
    pub options: Vec<JudgeClassificationOutputOption>,
}

impl JudgeClassificationOutput {
    pub fn builder() -> JudgeClassificationOutputBuilder {
        <JudgeClassificationOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeClassificationOutputBuilder {
    options: Option<Vec<JudgeClassificationOutputOption>>,
}

impl JudgeClassificationOutputBuilder {
    pub fn options(mut self, value: Vec<JudgeClassificationOutputOption>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeClassificationOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`options`](JudgeClassificationOutputBuilder::options)
    pub fn build(self) -> Result<JudgeClassificationOutput, BuildError> {
        Ok(JudgeClassificationOutput {
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
        })
    }
}
