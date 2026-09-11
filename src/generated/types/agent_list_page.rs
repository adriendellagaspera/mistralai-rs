#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentListPage {
    pub data: Vec<Agent>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next_page_token: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentListPageObject>,
}
