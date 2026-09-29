pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationLlmv2CategoryThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hate_and_discrimination: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violence_and_threats: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerous: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criminal: Option<f64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jailbreaking: Option<f64>,
}

impl ModerationLlmv2CategoryThresholds {
    pub fn builder() -> ModerationLlmv2CategoryThresholdsBuilder {
        <ModerationLlmv2CategoryThresholdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationLlmv2CategoryThresholdsBuilder {
    sexual: Option<f64>,
    hate_and_discrimination: Option<f64>,
    violence_and_threats: Option<f64>,
    dangerous: Option<f64>,
    criminal: Option<f64>,
    selfharm: Option<f64>,
    health: Option<f64>,
    financial: Option<f64>,
    law: Option<f64>,
    pii: Option<f64>,
    jailbreaking: Option<f64>,
}

impl ModerationLlmv2CategoryThresholdsBuilder {
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

    pub fn dangerous(mut self, value: f64) -> Self {
        self.dangerous = Some(value);
        self
    }

    pub fn criminal(mut self, value: f64) -> Self {
        self.criminal = Some(value);
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

    pub fn jailbreaking(mut self, value: f64) -> Self {
        self.jailbreaking = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationLlmv2CategoryThresholds`].
    pub fn build(self) -> Result<ModerationLlmv2CategoryThresholds, BuildError> {
        Ok(ModerationLlmv2CategoryThresholds {
            sexual: self.sexual,
            hate_and_discrimination: self.hate_and_discrimination,
            violence_and_threats: self.violence_and_threats,
            dangerous: self.dangerous,
            criminal: self.criminal,
            selfharm: self.selfharm,
            health: self.health,
            financial: self.financial,
            law: self.law,
            pii: self.pii,
            jailbreaking: self.jailbreaking,
        })
    }
}
