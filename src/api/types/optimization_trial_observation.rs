pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OptimizationTrialObservation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_type: Option<String>,
    #[serde(default)]
    pub run: EvaluationRunV2Response,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

impl OptimizationTrialObservation {
    pub fn builder() -> OptimizationTrialObservationBuilder {
        <OptimizationTrialObservationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptimizationTrialObservationBuilder {
    metadata: Option<HashMap<String, JsonValue>>,
    observation_type: Option<String>,
    run: Option<EvaluationRunV2Response>,
    score: Option<f64>,
}

impl OptimizationTrialObservationBuilder {
    pub fn metadata(mut self, value: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn observation_type(mut self, value: impl Into<String>) -> Self {
        self.observation_type = Some(value.into());
        self
    }

    pub fn run(mut self, value: EvaluationRunV2Response) -> Self {
        self.run = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OptimizationTrialObservation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run`](OptimizationTrialObservationBuilder::run)
    pub fn build(self) -> Result<OptimizationTrialObservation, BuildError> {
        Ok(OptimizationTrialObservation {
            metadata: self.metadata,
            observation_type: self.observation_type,
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
            score: self.score,
        })
    }
}
