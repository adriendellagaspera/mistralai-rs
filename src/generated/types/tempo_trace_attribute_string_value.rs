#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceAttributeStringValue {
    ///The string value of the attribute
    #[serde(rename = "stringValue")]
    pub string_value: String,
}
