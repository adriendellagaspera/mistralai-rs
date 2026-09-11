///The value of the attribute
#[derive(Debug, Clone)]
pub enum TempoTraceAttributeValue {
    TempoTraceAttributeStringValue(TempoTraceAttributeStringValue),
    TempoTraceAttributeIntValue(TempoTraceAttributeIntValue),
    TempoTraceAttributeBoolValue(TempoTraceAttributeBoolValue),
}
