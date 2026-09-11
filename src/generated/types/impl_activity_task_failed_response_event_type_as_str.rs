impl ActivityTaskFailedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivityTaskFailed => "ACTIVITY_TASK_FAILED",
        }
    }
}
