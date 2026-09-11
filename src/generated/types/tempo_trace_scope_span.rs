#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceScopeSpan {
    pub scope: TempoTraceScope,
    ///The spans of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<TempoTraceSpan>>,
}
