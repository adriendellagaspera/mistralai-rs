impl CredentialsCreateOrUpdateBuilder {
    /// Start a builder with every required wire field.
    pub fn new(name: String) -> Self {
        Self {
            value: CredentialsCreateOrUpdate::new(name),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "credentials", "` request field to a value."
    )]
    #[must_use]
    pub fn credentials(mut self, credentials: ConnectionCredentials) -> Self {
        self.value.credentials = Some(Some(credentials));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "credentials", "` request field to JSON null."
    )]
    #[must_use]
    pub fn credentials_null(mut self) -> Self {
        self.value.credentials = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "credentials", "` request field.")]
    #[must_use]
    pub fn credentials_absent(mut self) -> Self {
        self.value.credentials = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "is_default", "` request field to a value."
    )]
    #[must_use]
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.value.is_default = Some(Some(is_default));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "is_default", "` request field to JSON null."
    )]
    #[must_use]
    pub fn is_default_null(mut self) -> Self {
        self.value.is_default = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "is_default", "` request field.")]
    #[must_use]
    pub fn is_default_absent(mut self) -> Self {
        self.value.is_default = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> CredentialsCreateOrUpdate {
        self.value
    }
}
