pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OptimizationTrialWithObservations {
    #[serde(default)]
    pub algorithm_trial_key: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypothesis: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<OptimizationTrialObservation>>,
    #[serde(default)]
    pub optimization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_trial_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl OptimizationTrialWithObservations {
    pub fn builder() -> OptimizationTrialWithObservationsBuilder {
        <OptimizationTrialWithObservationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptimizationTrialWithObservationsBuilder {
    algorithm_trial_key: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    hypothesis: Option<String>,
    id: Option<String>,
    metadata: Option<HashMap<String, JsonValue>>,
    observations: Option<Vec<OptimizationTrialObservation>>,
    optimization_id: Option<String>,
    parent_trial_id: Option<String>,
    resolution: Option<String>,
    status: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl OptimizationTrialWithObservationsBuilder {
    pub fn algorithm_trial_key(mut self, value: impl Into<String>) -> Self {
        self.algorithm_trial_key = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn hypothesis(mut self, value: impl Into<String>) -> Self {
        self.hypothesis = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn observations(mut self, value: Vec<OptimizationTrialObservation>) -> Self {
        self.observations = Some(value);
        self
    }

    pub fn optimization_id(mut self, value: impl Into<String>) -> Self {
        self.optimization_id = Some(value.into());
        self
    }

    pub fn parent_trial_id(mut self, value: impl Into<String>) -> Self {
        self.parent_trial_id = Some(value.into());
        self
    }

    pub fn resolution(mut self, value: impl Into<String>) -> Self {
        self.resolution = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OptimizationTrialWithObservations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`algorithm_trial_key`](OptimizationTrialWithObservationsBuilder::algorithm_trial_key)
    /// - [`created_at`](OptimizationTrialWithObservationsBuilder::created_at)
    /// - [`id`](OptimizationTrialWithObservationsBuilder::id)
    /// - [`metadata`](OptimizationTrialWithObservationsBuilder::metadata)
    /// - [`optimization_id`](OptimizationTrialWithObservationsBuilder::optimization_id)
    /// - [`status`](OptimizationTrialWithObservationsBuilder::status)
    /// - [`updated_at`](OptimizationTrialWithObservationsBuilder::updated_at)
    pub fn build(self) -> Result<OptimizationTrialWithObservations, BuildError> {
        Ok(OptimizationTrialWithObservations {
            algorithm_trial_key: self
                .algorithm_trial_key
                .ok_or_else(|| BuildError::missing_field("algorithm_trial_key"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            hypothesis: self.hypothesis,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            observations: self.observations,
            optimization_id: self
                .optimization_id
                .ok_or_else(|| BuildError::missing_field("optimization_id"))?,
            parent_trial_id: self.parent_trial_id,
            resolution: self.resolution,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
