impl CustomTaskFailedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomTaskFailed => "CUSTOM_TASK_FAILED",
        }
    }
}
