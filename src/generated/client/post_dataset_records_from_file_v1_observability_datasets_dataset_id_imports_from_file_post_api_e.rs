///Typed error responses for `post_dataset_records_from_file_v1_observability_datasets__dataset_id__imports_from_file_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum PostDatasetRecordsFromFileV1ObservabilityDatasetsDatasetIdImportsFromFilePostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
