#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryInvocationBody {
    ///Input data for the query, matching its schema
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub input: Option<Option<QueryInvocationBodyInput>>,
    ///The name of the query to request
    pub name: String,
}
