pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationLlmv1CategoryThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hate_and_discrimination: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violence_and_threats: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerous_and_criminal_content: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfharm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub law: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<f64>,
}

impl ModerationLlmv1CategoryThresholds {
    pub fn builder() -> ModerationLlmv1CategoryThresholdsBuilder {
        <ModerationLlmv1CategoryThresholdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationLlmv1CategoryThresholdsBuilder {
    sexual: Option<f64>,
    hate_and_discrimination: Option<f64>,
    violence_and_threats: Option<f64>,
    dangerous_and_criminal_content: Option<f64>,
    selfharm: Option<f64>,
    health: Option<f64>,
    financial: Option<f64>,
    law: Option<f64>,
    pii: Option<f64>,
}

impl ModerationLlmv1CategoryThresholdsBuilder {
    pub fn sexual(mut self, value: f64) -> Self {
        self.sexual = Some(value);
        self
    }

    pub fn hate_and_discrimination(mut self, value: f64) -> Self {
        self.hate_and_discrimination = Some(value);
        self
    }

    pub fn violence_and_threats(mut self, value: f64) -> Self {
        self.violence_and_threats = Some(value);
        self
    }

    pub fn dangerous_and_criminal_content(mut self, value: f64) -> Self {
        self.dangerous_and_criminal_content = Some(value);
        self
    }

    pub fn selfharm(mut self, value: f64) -> Self {
        self.selfharm = Some(value);
        self
    }

    pub fn health(mut self, value: f64) -> Self {
        self.health = Some(value);
        self
    }

    pub fn financial(mut self, value: f64) -> Self {
        self.financial = Some(value);
        self
    }

    pub fn law(mut self, value: f64) -> Self {
        self.law = Some(value);
        self
    }

    pub fn pii(mut self, value: f64) -> Self {
        self.pii = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationLlmv1CategoryThresholds`].
    pub fn build(self) -> Result<ModerationLlmv1CategoryThresholds, BuildError> {
        Ok(ModerationLlmv1CategoryThresholds {
            sexual: self.sexual,
            hate_and_discrimination: self.hate_and_discrimination,
            violence_and_threats: self.violence_and_threats,
            dangerous_and_criminal_content: self.dangerous_and_criminal_content,
            selfharm: self.selfharm,
            health: self.health,
            financial: self.financial,
            law: self.law,
            pii: self.pii,
        })
    }
}
