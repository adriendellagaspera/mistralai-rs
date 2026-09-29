pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowScheduleRequest {
    /// The schedule definition
    pub schedule: ScheduleDefinition,
    /// The ID of the workflow registration to schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_registration_id: Option<String>,
    /// Deprecated: use workflow_registration_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_version_id: Option<String>,
    /// The name or ID of the workflow to schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_identifier: Option<String>,
    /// Deprecated. Use deployment_name instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_task_queue: Option<String>,
    /// Allows you to specify a custom schedule ID. If not provided, a random ID will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// Name of the deployment to route this schedule to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}

impl WorkflowScheduleRequest {
    pub fn builder() -> WorkflowScheduleRequestBuilder {
        <WorkflowScheduleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowScheduleRequestBuilder {
    schedule: Option<ScheduleDefinition>,
    workflow_registration_id: Option<String>,
    workflow_version_id: Option<String>,
    workflow_identifier: Option<String>,
    workflow_task_queue: Option<String>,
    schedule_id: Option<String>,
    deployment_name: Option<String>,
}

impl WorkflowScheduleRequestBuilder {
    pub fn schedule(mut self, value: ScheduleDefinition) -> Self {
        self.schedule = Some(value);
        self
    }

    pub fn workflow_registration_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_registration_id = Some(value.into());
        self
    }

    pub fn workflow_version_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_version_id = Some(value.into());
        self
    }

    pub fn workflow_identifier(mut self, value: impl Into<String>) -> Self {
        self.workflow_identifier = Some(value.into());
        self
    }

    pub fn workflow_task_queue(mut self, value: impl Into<String>) -> Self {
        self.workflow_task_queue = Some(value.into());
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowScheduleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule`](WorkflowScheduleRequestBuilder::schedule)
    pub fn build(self) -> Result<WorkflowScheduleRequest, BuildError> {
        Ok(WorkflowScheduleRequest {
            schedule: self
                .schedule
                .ok_or_else(|| BuildError::missing_field("schedule"))?,
            workflow_registration_id: self.workflow_registration_id,
            workflow_version_id: self.workflow_version_id,
            workflow_identifier: self.workflow_identifier,
            workflow_task_queue: self.workflow_task_queue,
            schedule_id: self.schedule_id,
            deployment_name: self.deployment_name,
        })
    }
}
