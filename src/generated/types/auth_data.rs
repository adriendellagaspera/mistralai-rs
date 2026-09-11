#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthData {
    pub client_id: String,
    pub client_secret: String,
}
