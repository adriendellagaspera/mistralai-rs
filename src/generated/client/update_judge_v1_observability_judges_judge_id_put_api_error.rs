///Typed error responses for `update_judge_v1_observability_judges__judge_id__put`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
