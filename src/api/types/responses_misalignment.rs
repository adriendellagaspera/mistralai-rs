pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesMisalignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steer: Option<ResponsesMisalignmentSteer>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesMisalignment {
    pub fn builder() -> ResponsesMisalignmentBuilder {
        <ResponsesMisalignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesMisalignmentBuilder {
    detailed_explanation: Option<String>,
    error_type: Option<String>,
    review_target: Option<String>,
    steer: Option<ResponsesMisalignmentSteer>,
}

impl ResponsesMisalignmentBuilder {
    pub fn detailed_explanation(mut self, value: impl Into<String>) -> Self {
        self.detailed_explanation = Some(value.into());
        self
    }

    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn review_target(mut self, value: impl Into<String>) -> Self {
        self.review_target = Some(value.into());
        self
    }

    pub fn steer(mut self, value: ResponsesMisalignmentSteer) -> Self {
        self.steer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesMisalignment`].
    pub fn build(self) -> Result<ResponsesMisalignment, BuildError> {
        Ok(ResponsesMisalignment {
            detailed_explanation: self.detailed_explanation,
            error_type: self.error_type,
            review_target: self.review_target,
            steer: self.steer,
            extra: Default::default(),
        })
    }
}
