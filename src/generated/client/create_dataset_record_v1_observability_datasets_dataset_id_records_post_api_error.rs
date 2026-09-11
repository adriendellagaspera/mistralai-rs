///Typed error responses for `create_dataset_record_v1_observability_datasets__dataset_id__records_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CreateDatasetRecordV1ObservabilityDatasetsDatasetIdRecordsPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
