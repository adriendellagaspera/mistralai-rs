///Typed error responses for `delete_dataset_record_v1_observability_dataset_records__dataset_record_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteDatasetRecordV1ObservabilityDatasetRecordsDatasetRecordIdDeleteApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
