#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowScheduleListResponse {
    ///A list of workflow schedules
    pub schedules: Vec<ScheduleDefinitionOutput>,
}
