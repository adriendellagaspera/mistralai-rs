///Typed error responses for `get_dataset_records_v1_observability_datasets__dataset_id__records_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
