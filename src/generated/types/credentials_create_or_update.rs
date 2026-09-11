///Request to create or update non-OAuth2 credentials for a connector.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialsCreateOrUpdate {
    ///The credential data (headers, bearer_token).
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub credentials: Option<Option<ConnectionCredentials>>,
    ///Controls whether this credential is the default for its auth method. On creation: if no credential exists yet for this auth method, the credential is automatically set as default when is_default is true or omitted; setting is_default to false is rejected because a default must exist. If other credentials already exist, setting is_default to true promotes this credential (demoting the previous default); false or omitted creates it as non-default. On update: true promotes this credential, false is rejected if it is currently the default (promote another credential first), omitted leaves the default status unchanged.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub is_default: Option<Option<bool>>,
    ///Name of the credentials. Use this name to access or modify your credentials.
    pub name: String,
}
