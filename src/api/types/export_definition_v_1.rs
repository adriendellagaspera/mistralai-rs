pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct ExportDefinitionV1 {
    pub destination: OtlpDestination,
}

impl ExportDefinitionV1 {
    pub fn builder() -> ExportDefinitionV1Builder {
        <ExportDefinitionV1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportDefinitionV1Builder {
    destination: Option<OtlpDestination>,
}

impl ExportDefinitionV1Builder {
    pub fn destination(mut self, value: OtlpDestination) -> Self {
        self.destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportDefinitionV1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](ExportDefinitionV1Builder::destination)
    pub fn build(self) -> Result<ExportDefinitionV1, BuildError> {
        Ok(ExportDefinitionV1 {
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
