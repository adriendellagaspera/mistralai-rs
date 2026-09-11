///Typed error responses for `judge_chat_completion_event_v1_observability_chat_completion_events__event_id__live_judging_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum JudgeChatCompletionEventV1ObservabilityChatCompletionEventsEventIdLiveJudgingPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
