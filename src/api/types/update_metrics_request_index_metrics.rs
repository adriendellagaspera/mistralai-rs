pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMetricsRequestIndexMetrics {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub document_count: i64,
}

impl UpdateMetricsRequestIndexMetrics {
    pub fn builder() -> UpdateMetricsRequestIndexMetricsBuilder {
        <UpdateMetricsRequestIndexMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMetricsRequestIndexMetricsBuilder {
    name: Option<String>,
    document_count: Option<i64>,
}

impl UpdateMetricsRequestIndexMetricsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMetricsRequestIndexMetrics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateMetricsRequestIndexMetricsBuilder::name)
    /// - [`document_count`](UpdateMetricsRequestIndexMetricsBuilder::document_count)
    pub fn build(self) -> Result<UpdateMetricsRequestIndexMetrics, BuildError> {
        Ok(UpdateMetricsRequestIndexMetrics {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            document_count: self
                .document_count
                .ok_or_else(|| BuildError::missing_field("document_count"))?,
        })
    }
}
