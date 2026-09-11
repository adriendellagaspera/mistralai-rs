///Typed error responses for `get_campaigns_v1_observability_campaigns_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetCampaignsV1ObservabilityCampaignsGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
