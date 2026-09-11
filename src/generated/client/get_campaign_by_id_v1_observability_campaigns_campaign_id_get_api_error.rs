///Typed error responses for `get_campaign_by_id_v1_observability_campaigns__campaign_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetCampaignByIdV1ObservabilityCampaignsCampaignIdGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
