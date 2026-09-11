#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignalInvocationBody {
    ///Input data for the signal, matching its schema
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub input: Option<Option<SignalInvocationBodyInput>>,
    ///The name of the signal to send
    pub name: String,
}
