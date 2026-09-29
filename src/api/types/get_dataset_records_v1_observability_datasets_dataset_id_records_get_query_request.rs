pub use crate::prelude::*;

/// Query parameters for get_dataset_records_v1_observability_datasets__dataset_id__records_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest {
    pub fn builder(
    ) -> GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequestBuilder {
        <GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest, BuildError>
    {
        Ok(
            GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest {
                page_size: self.page_size,
                page: self.page,
            },
        )
    }
}
