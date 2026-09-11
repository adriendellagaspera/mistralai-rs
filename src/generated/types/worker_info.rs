#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkerInfo {
    pub namespace: String,
    pub scheduler_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
}
