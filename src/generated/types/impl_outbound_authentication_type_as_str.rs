impl OutboundAuthenticationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Oauth2 => "oauth2",
            Self::Bearer => "bearer",
            Self::None => "none",
        }
    }
}
