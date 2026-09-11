#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthUrlResponse {
    pub auth_url: String,
    pub ttl: i64,
}
