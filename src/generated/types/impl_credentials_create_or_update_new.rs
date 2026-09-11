impl CredentialsCreateOrUpdate {
    /// Construct this request with every required wire field.
    pub fn new(name: String) -> Self {
        Self {
            name,
            credentials: None,
            is_default: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(name: String) -> CredentialsCreateOrUpdateBuilder {
        CredentialsCreateOrUpdateBuilder::new(name)
    }
}
