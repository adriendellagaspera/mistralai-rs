impl BaseTaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "RUNNING",
            Self::Completed => "COMPLETED",
            Self::Failed => "FAILED",
            Self::Canceled => "CANCELED",
            Self::Terminated => "TERMINATED",
            Self::ContinuedAsNew => "CONTINUED_AS_NEW",
            Self::TimedOut => "TIMED_OUT",
            Self::Unknown => "UNKNOWN",
        }
    }
}
