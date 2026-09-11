#[derive(Debug, Clone)]
pub enum WorkflowExecutionTraceEventsResponseEventsItemUnion {
    WorkflowExecutionTraceEvent(WorkflowExecutionTraceEvent),
    WorkflowExecutionProgressTraceEvent(WorkflowExecutionProgressTraceEvent),
}
