pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferCallTool {
    #[serde(default)]
    pub destinations: Vec<TransferTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<TransferCallToolFiller>,
}

impl TransferCallTool {
    pub fn builder() -> TransferCallToolBuilder {
        <TransferCallToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferCallToolBuilder {
    destinations: Option<Vec<TransferTarget>>,
    filler: Option<TransferCallToolFiller>,
}

impl TransferCallToolBuilder {
    pub fn destinations(mut self, value: Vec<TransferTarget>) -> Self {
        self.destinations = Some(value);
        self
    }

    pub fn filler(mut self, value: TransferCallToolFiller) -> Self {
        self.filler = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferCallTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destinations`](TransferCallToolBuilder::destinations)
    pub fn build(self) -> Result<TransferCallTool, BuildError> {
        Ok(TransferCallTool {
            destinations: self
                .destinations
                .ok_or_else(|| BuildError::missing_field("destinations"))?,
            filler: self.filler,
        })
    }
}
