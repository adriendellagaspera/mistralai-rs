pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum UpdateJudgeRequestOutput {
    #[serde(rename = "CLASSIFICATION")]
    #[non_exhaustive]
    Classification {
        #[serde(flatten)]
        data: JudgeClassificationOutput,
    },

    #[serde(rename = "REGRESSION")]
    #[non_exhaustive]
    Regression {
        #[serde(flatten)]
        data: JudgeRegressionOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl UpdateJudgeRequestOutput {
    pub fn classification(data: JudgeClassificationOutput) -> Self {
        Self::Classification { data }
    }

    pub fn regression(data: JudgeRegressionOutput) -> Self {
        Self::Regression { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
