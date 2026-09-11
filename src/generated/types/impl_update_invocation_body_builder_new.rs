impl UpdateInvocationBodyBuilder {
    /// Start a builder with every required wire field.
    pub fn new(name: String) -> Self {
        Self {
            value: UpdateInvocationBody::new(name),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "input", "` request field to a value."
    )]
    #[must_use]
    pub fn input(mut self, input: UpdateInvocationBodyInput) -> Self {
        self.value.input = Some(Some(input));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "input", "` request field to JSON null."
    )]
    #[must_use]
    pub fn input_null(mut self) -> Self {
        self.value.input = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "input", "` request field.")]
    #[must_use]
    pub fn input_absent(mut self) -> Self {
        self.value.input = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> UpdateInvocationBody {
        self.value
    }
}
