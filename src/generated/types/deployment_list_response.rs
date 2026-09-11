#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentListResponse {
    ///List of deployments
    pub deployments: Vec<DeploymentResponse>,
}
