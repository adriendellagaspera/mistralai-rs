///Typed error responses for `delete_dataset_records_v1_observability_dataset_records_bulk_delete_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteDatasetRecordsV1ObservabilityDatasetRecordsBulkDeletePostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
