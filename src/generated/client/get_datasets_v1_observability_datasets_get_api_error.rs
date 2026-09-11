///Typed error responses for `get_datasets_v1_observability_datasets_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDatasetsV1ObservabilityDatasetsGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
