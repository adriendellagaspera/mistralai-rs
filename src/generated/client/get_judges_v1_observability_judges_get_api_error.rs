///Typed error responses for `get_judges_v1_observability_judges_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetJudgesV1ObservabilityJudgesGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
