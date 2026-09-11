#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostCampaignInSchema {
    pub description: String,
    pub judge_id: uuid::Uuid,
    ///Constraint: maximum=10000, exclusiveMinimum=0
    pub max_nb_events: i64,
    ///Constraint: minLength=5, maxLength=50
    pub name: String,
    pub search_params: FilterPayload,
}
