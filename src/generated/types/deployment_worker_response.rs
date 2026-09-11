#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentWorkerResponse {
    ///When the worker first registered
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Worker name
    pub name: String,
    ///When the worker last registered
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
