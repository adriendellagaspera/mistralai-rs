impl ProcessStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SelfManaged => "self_managed",
            Self::MissingContent => "missing_content",
            Self::Noop => "noop",
            Self::Done => "done",
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Error => "error",
            Self::WaitingForCapacity => "waiting_for_capacity",
        }
    }
}
