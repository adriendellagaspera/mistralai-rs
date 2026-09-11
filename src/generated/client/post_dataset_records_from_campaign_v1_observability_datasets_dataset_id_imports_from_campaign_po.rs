///Typed error responses for `post_dataset_records_from_campaign_v1_observability_datasets__dataset_id__imports_from_campaign_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum PostDatasetRecordsFromCampaignV1ObservabilityDatasetsDatasetIdImportsFromCampaignPostApiError
{
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
