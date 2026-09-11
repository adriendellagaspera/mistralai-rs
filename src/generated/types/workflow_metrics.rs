/**Complete metrics for a specific workflow.

This type combines all metric categories.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowMetrics {
    pub average_latency_ms: ScalarMetric,
    pub error_count: ScalarMetric,
    pub execution_count: ScalarMetric,
    pub latency_over_time: TimeSeriesMetric,
    pub retry_rate: ScalarMetric,
    pub success_count: ScalarMetric,
}
