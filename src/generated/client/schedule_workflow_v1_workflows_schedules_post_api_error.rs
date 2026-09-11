///Typed error responses for `schedule_workflow_v1_workflows_schedules_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ScheduleWorkflowV1WorkflowsSchedulesPostApiError {
    Status422(HTTPValidationError),
}
