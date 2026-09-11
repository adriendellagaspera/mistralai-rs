impl OAuth2TokenTokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bearer => "Bearer",
        }
    }
}
