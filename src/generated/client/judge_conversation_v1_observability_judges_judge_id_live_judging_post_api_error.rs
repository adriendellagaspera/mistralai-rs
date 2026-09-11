///Typed error responses for `judge_conversation_v1_observability_judges__judge_id__live_judging_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum JudgeConversationV1ObservabilityJudgesJudgeIdLiveJudgingPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
