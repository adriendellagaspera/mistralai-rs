#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowScheduleResponse {
    ///The ID of the schedule
    pub schedule_id: String,
}
