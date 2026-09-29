pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMetricsRequestIndexMetrics {
    #[serde(default)]
    pub document_count: i64,
    #[serde(default)]
    pub name: String,
}

impl UpdateMetricsRequestIndexMetrics {
    pub fn builder() -> UpdateMetricsRequestIndexMetricsBuilder {
        <UpdateMetricsRequestIndexMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMetricsRequestIndexMetricsBuilder {
    document_count: Option<i64>,
    name: Option<String>,
}

impl UpdateMetricsRequestIndexMetricsBuilder {
    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMetricsRequestIndexMetrics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_count`](UpdateMetricsRequestIndexMetricsBuilder::document_count)
    /// - [`name`](UpdateMetricsRequestIndexMetricsBuilder::name)
    pub fn build(self) -> Result<UpdateMetricsRequestIndexMetrics, BuildError> {
        Ok(UpdateMetricsRequestIndexMetrics {
            document_count: self
                .document_count
                .ok_or_else(|| BuildError::missing_field("document_count"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
