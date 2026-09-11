///Typed error responses for `get_judge_by_id_v1_observability_judges__judge_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetJudgeByIdV1ObservabilityJudgesJudgeIdGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
