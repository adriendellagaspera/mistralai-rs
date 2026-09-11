impl TempoTraceScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SpanKindInternal => "SPAN_KIND_INTERNAL",
            Self::SpanKindServer => "SPAN_KIND_SERVER",
            Self::SpanKindClient => "SPAN_KIND_CLIENT",
        }
    }
}
