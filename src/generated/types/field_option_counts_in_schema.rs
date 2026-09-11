#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FieldOptionCountsInSchema {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub filter_params: Option<Option<FilterPayload>>,
}
