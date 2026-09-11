impl ActivityTaskStartedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivityTaskStarted => "ACTIVITY_TASK_STARTED",
        }
    }
}
