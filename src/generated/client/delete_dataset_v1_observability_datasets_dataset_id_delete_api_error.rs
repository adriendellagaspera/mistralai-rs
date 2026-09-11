///Typed error responses for `delete_dataset_v1_observability_datasets__dataset_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteDatasetV1ObservabilityDatasetsDatasetIdDeleteApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
