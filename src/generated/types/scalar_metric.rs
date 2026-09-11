///Scalar metric with a single value.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScalarMetric {
    pub value: ScalarMetricValue,
}
