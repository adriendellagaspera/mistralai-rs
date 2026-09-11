#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceAttribute {
    ///The key of the attribute
    pub key: String,
    ///The value of the attribute
    pub value: TempoTraceAttributeValue,
}
