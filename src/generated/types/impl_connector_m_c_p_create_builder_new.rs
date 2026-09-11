impl ConnectorMCPCreateBuilder {
    /// Start a builder with every required wire field.
    pub fn new(description: String, name: String, server: url::Url) -> Self {
        Self {
            value: ConnectorMCPCreate::new(description, name, server),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "auth_data", "` request field to a value."
    )]
    #[must_use]
    pub fn auth_data(mut self, auth_data: AuthData) -> Self {
        self.value.auth_data = Some(Some(auth_data));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "auth_data", "` request field to JSON null."
    )]
    #[must_use]
    pub fn auth_data_null(mut self) -> Self {
        self.value.auth_data = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "auth_data", "` request field.")]
    #[must_use]
    pub fn auth_data_absent(mut self) -> Self {
        self.value.auth_data = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "headers", "` request field to a value."
    )]
    #[must_use]
    pub fn headers(mut self, headers: ConnectorMCPCreateHeaders) -> Self {
        self.value.headers = Some(Some(headers));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "headers", "` request field to JSON null."
    )]
    #[must_use]
    pub fn headers_null(mut self) -> Self {
        self.value.headers = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "headers", "` request field.")]
    #[must_use]
    pub fn headers_absent(mut self) -> Self {
        self.value.headers = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "icon_url", "` request field to a value."
    )]
    #[must_use]
    pub fn icon_url(mut self, icon_url: String) -> Self {
        self.value.icon_url = Some(Some(icon_url));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "icon_url", "` request field to JSON null."
    )]
    #[must_use]
    pub fn icon_url_null(mut self) -> Self {
        self.value.icon_url = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "icon_url", "` request field.")]
    #[must_use]
    pub fn icon_url_absent(mut self) -> Self {
        self.value.icon_url = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "system_prompt", "` request field to a value."
    )]
    #[must_use]
    pub fn system_prompt(mut self, system_prompt: String) -> Self {
        self.value.system_prompt = Some(Some(system_prompt));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "system_prompt", "` request field to JSON null."
    )]
    #[must_use]
    pub fn system_prompt_null(mut self) -> Self {
        self.value.system_prompt = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "system_prompt", "` request field.")]
    #[must_use]
    pub fn system_prompt_absent(mut self) -> Self {
        self.value.system_prompt = None;
        self
    }
    #[doc = concat!("Set the optional `", "visibility", "` request field.")]
    #[must_use]
    pub fn visibility(mut self, visibility: ResourceVisibility) -> Self {
        self.value.visibility = Some(visibility);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> ConnectorMCPCreate {
        self.value
    }
}
