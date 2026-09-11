impl ResetInvocationBody {
    /// Construct this request with every required wire field.
    pub fn new(event_id: i64) -> Self {
        Self {
            event_id,
            exclude_signals: None,
            exclude_updates: None,
            reason: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(event_id: i64) -> ResetInvocationBodyBuilder {
        ResetInvocationBodyBuilder::new(event_id)
    }
}
