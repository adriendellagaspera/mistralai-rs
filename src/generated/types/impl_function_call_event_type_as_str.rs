impl FunctionCallEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FunctionCallDelta => "function.call.delta",
        }
    }
}
