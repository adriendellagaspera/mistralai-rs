#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceAttributeBoolValue {
    ///The boolean value of the attribute
    #[serde(rename = "boolValue")]
    pub bool_value: bool,
}
