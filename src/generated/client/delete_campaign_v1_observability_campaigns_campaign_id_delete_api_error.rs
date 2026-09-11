///Typed error responses for `delete_campaign_v1_observability_campaigns__campaign_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteCampaignV1ObservabilityCampaignsCampaignIdDeleteApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
