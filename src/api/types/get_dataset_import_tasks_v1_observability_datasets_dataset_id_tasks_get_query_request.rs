pub use crate::prelude::*;

/// Query parameters for get_dataset_import_tasks_v1_observability_datasets__dataset_id__tasks_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest {
    pub fn builder(
    ) -> GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequestBuilder {
        <GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest, BuildError>
    {
        Ok(
            GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest {
                page_size: self.page_size,
                page: self.page,
            },
        )
    }
}
