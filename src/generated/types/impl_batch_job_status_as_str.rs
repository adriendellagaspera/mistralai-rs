impl BatchJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "QUEUED",
            Self::Running => "RUNNING",
            Self::Success => "SUCCESS",
            Self::Failed => "FAILED",
            Self::TimeoutExceeded => "TIMEOUT_EXCEEDED",
            Self::CancellationRequested => "CANCELLATION_REQUESTED",
            Self::Cancelled => "CANCELLED",
        }
    }
}
