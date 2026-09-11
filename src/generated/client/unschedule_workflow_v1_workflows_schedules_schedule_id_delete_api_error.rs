///Typed error responses for `unschedule_workflow_v1_workflows_schedules__schedule_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UnscheduleWorkflowV1WorkflowsSchedulesScheduleIdDeleteApiError {
    Status422(HTTPValidationError),
}
