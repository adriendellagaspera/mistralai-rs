///Public view of an authentication method, without secrets.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicAuthenticationMethod {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub headers: Option<Option<Vec<ConnectorAuthenticationHeader>>>,
    pub method_type: OutboundAuthenticationType,
}
