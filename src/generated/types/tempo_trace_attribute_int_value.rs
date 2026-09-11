#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceAttributeIntValue {
    ///The integer value of the attribute
    #[serde(rename = "intValue")]
    pub int_value: String,
}
