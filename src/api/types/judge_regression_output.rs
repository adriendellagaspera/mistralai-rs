pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JudgeRegressionOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min: Option<f64>,
    #[serde(default)]
    pub min_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max: Option<f64>,
    #[serde(default)]
    pub max_description: String,
}

impl JudgeRegressionOutput {
    pub fn builder() -> JudgeRegressionOutputBuilder {
        <JudgeRegressionOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeRegressionOutputBuilder {
    min: Option<f64>,
    min_description: Option<String>,
    max: Option<f64>,
    max_description: Option<String>,
}

impl JudgeRegressionOutputBuilder {
    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn min_description(mut self, value: impl Into<String>) -> Self {
        self.min_description = Some(value.into());
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn max_description(mut self, value: impl Into<String>) -> Self {
        self.max_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JudgeRegressionOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`min_description`](JudgeRegressionOutputBuilder::min_description)
    /// - [`max_description`](JudgeRegressionOutputBuilder::max_description)
    pub fn build(self) -> Result<JudgeRegressionOutput, BuildError> {
        Ok(JudgeRegressionOutput {
            min: self.min,
            min_description: self
                .min_description
                .ok_or_else(|| BuildError::missing_field("min_description"))?,
            max: self.max,
            max_description: self
                .max_description
                .ok_or_else(|| BuildError::missing_field("max_description"))?,
        })
    }
}
