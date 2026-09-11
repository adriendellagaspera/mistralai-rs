#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CampaignSelectedEvents {
    pub completion_events: PaginatedResultChatCompletionEventPreview,
}
