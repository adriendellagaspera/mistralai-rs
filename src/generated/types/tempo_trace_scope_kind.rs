#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TempoTraceScopeKind {
    #[default]
    #[serde(rename = "SPAN_KIND_INTERNAL")]
    SpanKindInternal,
    #[serde(rename = "SPAN_KIND_SERVER")]
    SpanKindServer,
    #[serde(rename = "SPAN_KIND_CLIENT")]
    SpanKindClient,
}
