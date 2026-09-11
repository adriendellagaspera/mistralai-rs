#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScalarMetricValue {
    Integer(i64),
    Number(f64),
}
