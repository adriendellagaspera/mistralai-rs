///Typed error responses for `create_campaign_v1_observability_campaigns_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CreateCampaignV1ObservabilityCampaignsPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
