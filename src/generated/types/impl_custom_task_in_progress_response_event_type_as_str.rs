impl CustomTaskInProgressResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomTaskInProgress => "CUSTOM_TASK_IN_PROGRESS",
        }
    }
}
