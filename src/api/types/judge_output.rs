pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JudgeOutput {
    #[serde(default)]
    pub analysis: String,
    pub answer: JudgeOutputAnswer,
}

impl JudgeOutput {
    pub fn builder() -> JudgeOutputBuilder {
        <JudgeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeOutputBuilder {
    analysis: Option<String>,
    answer: Option<JudgeOutputAnswer>,
}

impl JudgeOutputBuilder {
    pub fn analysis(mut self, value: impl Into<String>) -> Self {
        self.analysis = Some(value.into());
        self
    }

    pub fn answer(mut self, value: JudgeOutputAnswer) -> Self {
        self.answer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis`](JudgeOutputBuilder::analysis)
    /// - [`answer`](JudgeOutputBuilder::answer)
    pub fn build(self) -> Result<JudgeOutput, BuildError> {
        Ok(JudgeOutput {
            analysis: self
                .analysis
                .ok_or_else(|| BuildError::missing_field("analysis"))?,
            answer: self
                .answer
                .ok_or_else(|| BuildError::missing_field("answer"))?,
        })
    }
}
