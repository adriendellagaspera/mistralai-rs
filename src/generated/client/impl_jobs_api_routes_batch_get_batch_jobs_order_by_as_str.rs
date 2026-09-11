impl JobsApiRoutesBatchGetBatchJobsOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Created2 => "-created",
        }
    }
}
