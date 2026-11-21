pub use crate::prelude::*;

/// One allowed hand-off, chosen by the model through `id` alone.
///
/// The address never reaches the model: it only sees `id` and `condition`, so it
/// cannot dial a near-miss of a real number.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferTarget {
    #[serde(default)]
    pub condition: String,
    pub destination: TransferTargetDestination,
    #[serde(default)]
    pub id: String,
}

impl TransferTarget {
    pub fn builder() -> TransferTargetBuilder {
        <TransferTargetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferTargetBuilder {
    condition: Option<String>,
    destination: Option<TransferTargetDestination>,
    id: Option<String>,
}

impl TransferTargetBuilder {
    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    pub fn destination(mut self, value: TransferTargetDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferTarget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](TransferTargetBuilder::condition)
    /// - [`destination`](TransferTargetBuilder::destination)
    /// - [`id`](TransferTargetBuilder::id)
    pub fn build(self) -> Result<TransferTarget, BuildError> {
        Ok(TransferTarget {
            condition: self
                .condition
                .ok_or_else(|| BuildError::missing_field("condition"))?,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
