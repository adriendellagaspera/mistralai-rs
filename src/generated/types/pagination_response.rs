#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationResponse {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next_cursor: Option<Option<String>>,
    pub page_size: i64,
}
