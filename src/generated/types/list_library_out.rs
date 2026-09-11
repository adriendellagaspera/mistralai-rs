#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListLibraryOut {
    pub data: Vec<LibraryOut>,
}
