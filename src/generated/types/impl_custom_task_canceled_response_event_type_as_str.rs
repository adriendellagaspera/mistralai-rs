impl CustomTaskCanceledResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomTaskCanceled => "CUSTOM_TASK_CANCELED",
        }
    }
}
