#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowScheduleRequest {
    ///Name of the deployment to route this schedule to
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub deployment_name: Option<Option<String>>,
    pub schedule: ScheduleDefinition,
    ///Allows you to specify a custom schedule ID. If not provided, a random ID will be generated.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub schedule_id: Option<Option<String>>,
    ///The name or ID of the workflow to schedule
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub workflow_identifier: Option<Option<String>>,
    ///The ID of the workflow registration to schedule
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub workflow_registration_id: Option<Option<uuid::Uuid>>,
    ///Deprecated. Use deployment_name instead.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub workflow_task_queue: Option<Option<String>>,
    ///Deprecated: use workflow_registration_id
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub workflow_version_id: Option<Option<uuid::Uuid>>,
}
