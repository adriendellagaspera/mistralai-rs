impl CustomTaskTimedOutResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomTaskTimedOut => "CUSTOM_TASK_TIMED_OUT",
        }
    }
}
