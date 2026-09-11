impl ResetInvocationBodyBuilder {
    /// Start a builder with every required wire field.
    pub fn new(event_id: i64) -> Self {
        Self {
            value: ResetInvocationBody::new(event_id),
        }
    }
    #[doc = concat!("Set the optional `", "exclude_signals", "` request field.")]
    #[must_use]
    pub fn exclude_signals(mut self, exclude_signals: bool) -> Self {
        self.value.exclude_signals = Some(exclude_signals);
        self
    }
    #[doc = concat!("Set the optional `", "exclude_updates", "` request field.")]
    #[must_use]
    pub fn exclude_updates(mut self, exclude_updates: bool) -> Self {
        self.value.exclude_updates = Some(exclude_updates);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "reason", "` request field to a value."
    )]
    #[must_use]
    pub fn reason(mut self, reason: String) -> Self {
        self.value.reason = Some(Some(reason));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "reason", "` request field to JSON null."
    )]
    #[must_use]
    pub fn reason_null(mut self) -> Self {
        self.value.reason = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "reason", "` request field.")]
    #[must_use]
    pub fn reason_absent(mut self) -> Self {
        self.value.reason = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> ResetInvocationBody {
        self.value
    }
}
