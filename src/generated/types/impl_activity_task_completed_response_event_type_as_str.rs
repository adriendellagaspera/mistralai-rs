impl ActivityTaskCompletedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivityTaskCompleted => "ACTIVITY_TASK_COMPLETED",
        }
    }
}
