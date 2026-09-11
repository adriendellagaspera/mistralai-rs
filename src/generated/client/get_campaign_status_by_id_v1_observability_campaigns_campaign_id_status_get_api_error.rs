///Typed error responses for `get_campaign_status_by_id_v1_observability_campaigns__campaign_id__status_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetCampaignStatusByIdV1ObservabilityCampaignsCampaignIdStatusGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
