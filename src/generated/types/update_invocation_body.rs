#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateInvocationBody {
    ///Input data for the update, matching its schema
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub input: Option<Option<UpdateInvocationBodyInput>>,
    ///The name of the update to request
    pub name: String,
}
