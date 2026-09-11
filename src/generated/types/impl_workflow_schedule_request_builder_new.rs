impl WorkflowScheduleRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(schedule: ScheduleDefinition) -> Self {
        Self {
            value: WorkflowScheduleRequest::new(schedule),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "deployment_name", "` request field to a value."
    )]
    #[must_use]
    pub fn deployment_name(mut self, deployment_name: String) -> Self {
        self.value.deployment_name = Some(Some(deployment_name));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "deployment_name", "` request field to JSON null."
    )]
    #[must_use]
    pub fn deployment_name_null(mut self) -> Self {
        self.value.deployment_name = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "deployment_name", "` request field."
    )]
    #[must_use]
    pub fn deployment_name_absent(mut self) -> Self {
        self.value.deployment_name = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "schedule_id", "` request field to a value."
    )]
    #[must_use]
    pub fn schedule_id(mut self, schedule_id: String) -> Self {
        self.value.schedule_id = Some(Some(schedule_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "schedule_id", "` request field to JSON null."
    )]
    #[must_use]
    pub fn schedule_id_null(mut self) -> Self {
        self.value.schedule_id = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "schedule_id", "` request field.")]
    #[must_use]
    pub fn schedule_id_absent(mut self) -> Self {
        self.value.schedule_id = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_identifier",
        "` request field to a value."
    )]
    #[must_use]
    pub fn workflow_identifier(mut self, workflow_identifier: String) -> Self {
        self.value.workflow_identifier = Some(Some(workflow_identifier));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_identifier",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn workflow_identifier_null(mut self) -> Self {
        self.value.workflow_identifier = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "workflow_identifier", "` request field."
    )]
    #[must_use]
    pub fn workflow_identifier_absent(mut self) -> Self {
        self.value.workflow_identifier = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_registration_id",
        "` request field to a value."
    )]
    #[must_use]
    pub fn workflow_registration_id(mut self, workflow_registration_id: uuid::Uuid) -> Self {
        self.value.workflow_registration_id = Some(Some(workflow_registration_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_registration_id",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn workflow_registration_id_null(mut self) -> Self {
        self.value.workflow_registration_id = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "workflow_registration_id", "` request field."
    )]
    #[must_use]
    pub fn workflow_registration_id_absent(mut self) -> Self {
        self.value.workflow_registration_id = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_task_queue",
        "` request field to a value."
    )]
    #[must_use]
    pub fn workflow_task_queue(mut self, workflow_task_queue: String) -> Self {
        self.value.workflow_task_queue = Some(Some(workflow_task_queue));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_task_queue",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn workflow_task_queue_null(mut self) -> Self {
        self.value.workflow_task_queue = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "workflow_task_queue", "` request field."
    )]
    #[must_use]
    pub fn workflow_task_queue_absent(mut self) -> Self {
        self.value.workflow_task_queue = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_version_id",
        "` request field to a value."
    )]
    #[must_use]
    pub fn workflow_version_id(mut self, workflow_version_id: uuid::Uuid) -> Self {
        self.value.workflow_version_id = Some(Some(workflow_version_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "workflow_version_id",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn workflow_version_id_null(mut self) -> Self {
        self.value.workflow_version_id = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "workflow_version_id", "` request field."
    )]
    #[must_use]
    pub fn workflow_version_id_absent(mut self) -> Self {
        self.value.workflow_version_id = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> WorkflowScheduleRequest {
        self.value
    }
}
