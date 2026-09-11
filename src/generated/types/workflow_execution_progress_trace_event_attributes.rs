///The attributes of the event
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowExecutionProgressTraceEventAttributes {
    /// Additional properties matching the spec's
    /// `additionalProperties` value schema.
    #[serde(flatten)]
    pub additional_properties:
        std::collections::BTreeMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
}
