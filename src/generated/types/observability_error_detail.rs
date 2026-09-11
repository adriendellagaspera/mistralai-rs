#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObservabilityErrorDetail {
    pub error_code: Option<ObservabilityErrorCode>,
    pub message: String,
}
