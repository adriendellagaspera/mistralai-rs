impl OAuth2TokenAuthType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Oauth2Token => "oauth2-token",
        }
    }
}
