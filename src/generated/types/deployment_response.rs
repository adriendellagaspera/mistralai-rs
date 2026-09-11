#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentResponse {
    ///When the deployment was first registered
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Unique identifier of the deployment
    pub id: uuid::Uuid,
    ///Whether at least one worker is currently live
    pub is_active: bool,
    ///Deployment name
    pub name: String,
    ///When the deployment was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
