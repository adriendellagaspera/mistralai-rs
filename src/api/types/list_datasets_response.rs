pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetsResponse {
    #[serde(default)]
    pub datasets: PaginatedResultDatasetPreview,
}

impl ListDatasetsResponse {
    pub fn builder() -> ListDatasetsResponseBuilder {
        <ListDatasetsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetsResponseBuilder {
    datasets: Option<PaginatedResultDatasetPreview>,
}

impl ListDatasetsResponseBuilder {
    pub fn datasets(mut self, value: PaginatedResultDatasetPreview) -> Self {
        self.datasets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`datasets`](ListDatasetsResponseBuilder::datasets)
    pub fn build(self) -> Result<ListDatasetsResponse, BuildError> {
        Ok(ListDatasetsResponse {
            datasets: self
                .datasets
                .ok_or_else(|| BuildError::missing_field("datasets"))?,
        })
    }
}
