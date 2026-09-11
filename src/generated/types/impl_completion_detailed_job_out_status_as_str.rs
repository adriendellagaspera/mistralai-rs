impl CompletionDetailedJobOutStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "QUEUED",
            Self::Started => "STARTED",
            Self::Validating => "VALIDATING",
            Self::Validated => "VALIDATED",
            Self::Running => "RUNNING",
            Self::FailedValidation => "FAILED_VALIDATION",
            Self::Failed => "FAILED",
            Self::Success => "SUCCESS",
            Self::Cancelled => "CANCELLED",
            Self::CancellationRequested => "CANCELLATION_REQUESTED",
        }
    }
}
