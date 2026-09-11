#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValidationErrorLocItemUnion {
    String(String),
    Integer(i64),
}
