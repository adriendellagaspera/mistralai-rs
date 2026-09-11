#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JudgeOutputAnswer {
    String(String),
    Number(f64),
}
