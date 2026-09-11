#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceBatch {
    pub resource: TempoTraceResource,
    ///The spans of the scope
    #[serde(rename = "scopeSpans", skip_serializing_if = "Option::is_none")]
    pub scope_spans: Option<Vec<TempoTraceScopeSpan>>,
}
