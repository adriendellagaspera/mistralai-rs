impl CustomTaskCompletedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomTaskCompleted => "CUSTOM_TASK_COMPLETED",
        }
    }
}
