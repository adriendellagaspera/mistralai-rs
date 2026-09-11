#[derive(Debug, Clone)]
pub enum CustomConnectorAuthorizationInline {
    OAuth2TokenAuth(OAuth2TokenAuth),
    APIKeyAuth(APIKeyAuth),
}
