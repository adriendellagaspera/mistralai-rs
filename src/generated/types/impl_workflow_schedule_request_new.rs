impl WorkflowScheduleRequest {
    /// Construct this request with every required wire field.
    pub fn new(schedule: ScheduleDefinition) -> Self {
        Self {
            schedule,
            deployment_name: None,
            schedule_id: None,
            workflow_identifier: None,
            workflow_registration_id: None,
            workflow_task_queue: None,
            workflow_version_id: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(schedule: ScheduleDefinition) -> WorkflowScheduleRequestBuilder {
        WorkflowScheduleRequestBuilder::new(schedule)
    }
}
