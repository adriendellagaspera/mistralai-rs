#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TimeSeriesMetricValueItem2Union {
    Integer(i64),
    Number(f64),
}
