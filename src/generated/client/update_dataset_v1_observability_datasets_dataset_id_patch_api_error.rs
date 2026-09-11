///Typed error responses for `update_dataset_v1_observability_datasets__dataset_id__patch`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateDatasetV1ObservabilityDatasetsDatasetIdPatchApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
