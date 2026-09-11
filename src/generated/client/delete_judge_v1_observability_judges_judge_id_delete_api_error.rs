///Typed error responses for `delete_judge_v1_observability_judges__judge_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteJudgeV1ObservabilityJudgesJudgeIdDeleteApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
