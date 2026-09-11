impl SignalInvocationBody {
    /// Construct this request with every required wire field.
    pub fn new(name: String) -> Self {
        Self { name, input: None }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(name: String) -> SignalInvocationBodyBuilder {
        SignalInvocationBodyBuilder::new(name)
    }
}
