impl ConnectorMCPCreate {
    /// Construct this request with every required wire field.
    pub fn new(description: String, name: String, server: url::Url) -> Self {
        Self {
            description,
            name,
            server,
            auth_data: None,
            headers: None,
            icon_url: None,
            system_prompt: None,
            visibility: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        description: String,
        name: String,
        server: url::Url,
    ) -> ConnectorMCPCreateBuilder {
        ConnectorMCPCreateBuilder::new(description, name, server)
    }
}
