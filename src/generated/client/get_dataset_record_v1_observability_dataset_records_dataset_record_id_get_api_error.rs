///Typed error responses for `get_dataset_record_v1_observability_dataset_records__dataset_record_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDatasetRecordV1ObservabilityDatasetRecordsDatasetRecordIdGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
