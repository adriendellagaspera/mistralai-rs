#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetChatCompletionEventIdsInSchema {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub extra_fields: Option<Option<Vec<String>>>,
    pub search_params: FilterPayload,
}
