///Typed error responses for `create_judge_v1_observability_judges_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CreateJudgeV1ObservabilityJudgesPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
