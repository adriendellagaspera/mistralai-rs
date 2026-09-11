#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObservabilityError {
    pub detail: ObservabilityErrorDetail,
}
