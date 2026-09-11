#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetImportFromCampaignInSchema {
    pub campaign_id: uuid::Uuid,
}
