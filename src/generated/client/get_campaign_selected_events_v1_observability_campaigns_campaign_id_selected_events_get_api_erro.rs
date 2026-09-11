///Typed error responses for `get_campaign_selected_events_v1_observability_campaigns__campaign_id__selected_events_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
