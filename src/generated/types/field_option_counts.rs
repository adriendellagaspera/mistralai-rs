#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldOptionCounts {
    pub counts: Vec<FieldOptionCountItem>,
}
