pub use crate::prelude::*;

/// Complete metrics for a specific workflow.
///
/// This type combines all metric categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowMetrics {
    pub average_latency_ms: ScalarMetric,
    pub error_count: ScalarMetric,
    pub execution_count: ScalarMetric,
    #[serde(default)]
    pub latency_over_time: TimeSeriesMetric,
    pub retry_rate: ScalarMetric,
    pub success_count: ScalarMetric,
}

impl WorkflowMetrics {
    pub fn builder() -> WorkflowMetricsBuilder {
        <WorkflowMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowMetricsBuilder {
    average_latency_ms: Option<ScalarMetric>,
    error_count: Option<ScalarMetric>,
    execution_count: Option<ScalarMetric>,
    latency_over_time: Option<TimeSeriesMetric>,
    retry_rate: Option<ScalarMetric>,
    success_count: Option<ScalarMetric>,
}

impl WorkflowMetricsBuilder {
    pub fn average_latency_ms(mut self, value: ScalarMetric) -> Self {
        self.average_latency_ms = Some(value);
        self
    }

    pub fn error_count(mut self, value: ScalarMetric) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn execution_count(mut self, value: ScalarMetric) -> Self {
        self.execution_count = Some(value);
        self
    }

    pub fn latency_over_time(mut self, value: TimeSeriesMetric) -> Self {
        self.latency_over_time = Some(value);
        self
    }

    pub fn retry_rate(mut self, value: ScalarMetric) -> Self {
        self.retry_rate = Some(value);
        self
    }

    pub fn success_count(mut self, value: ScalarMetric) -> Self {
        self.success_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowMetrics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_latency_ms`](WorkflowMetricsBuilder::average_latency_ms)
    /// - [`error_count`](WorkflowMetricsBuilder::error_count)
    /// - [`execution_count`](WorkflowMetricsBuilder::execution_count)
    /// - [`latency_over_time`](WorkflowMetricsBuilder::latency_over_time)
    /// - [`retry_rate`](WorkflowMetricsBuilder::retry_rate)
    /// - [`success_count`](WorkflowMetricsBuilder::success_count)
    pub fn build(self) -> Result<WorkflowMetrics, BuildError> {
        Ok(WorkflowMetrics {
            average_latency_ms: self
                .average_latency_ms
                .ok_or_else(|| BuildError::missing_field("average_latency_ms"))?,
            error_count: self
                .error_count
                .ok_or_else(|| BuildError::missing_field("error_count"))?,
            execution_count: self
                .execution_count
                .ok_or_else(|| BuildError::missing_field("execution_count"))?,
            latency_over_time: self
                .latency_over_time
                .ok_or_else(|| BuildError::missing_field("latency_over_time"))?,
            retry_rate: self
                .retry_rate
                .ok_or_else(|| BuildError::missing_field("retry_rate"))?,
            success_count: self
                .success_count
                .ok_or_else(|| BuildError::missing_field("success_count"))?,
        })
    }
}
