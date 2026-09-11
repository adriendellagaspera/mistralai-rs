#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TempoTraceResource {
    ///The attributes of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
}
