#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSharingOut {
    pub data: Vec<SharingOut>,
}
