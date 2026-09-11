///Time-series metric with timestamp-value pairs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeSeriesMetric {
    pub value: Vec<(i64, TimeSeriesMetricValueItem2Union)>,
}
