///Typed error responses for `create_dataset_v1_observability_datasets_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CreateDatasetV1ObservabilityDatasetsPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
