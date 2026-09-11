impl CustomConnectorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Connector => "connector",
        }
    }
}
