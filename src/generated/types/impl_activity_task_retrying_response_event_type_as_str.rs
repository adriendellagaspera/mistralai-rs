impl ActivityTaskRetryingResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivityTaskRetrying => "ACTIVITY_TASK_RETRYING",
        }
    }
}
