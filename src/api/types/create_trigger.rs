pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTrigger {
    /// Action to execute when the trigger fires
    pub action: CreateTriggerAction,
    /// ID of the event source that fires this trigger
    #[serde(default)]
    pub event_source_id: String,
    /// jmespath expression to filter events, e.g. event_type == 'push' && repository.name == 'dashboard'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    /// workflows, agents, or spaces
    pub handler: TriggerHandler,
    pub owner_type: TriggersOwnerType,
    /// Human-readable title of the trigger
    #[serde(default)]
    pub title: String,
}

impl CreateTrigger {
    pub fn builder() -> CreateTriggerBuilder {
        <CreateTriggerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTriggerBuilder {
    action: Option<CreateTriggerAction>,
    event_source_id: Option<String>,
    filters: Option<String>,
    handler: Option<TriggerHandler>,
    owner_type: Option<TriggersOwnerType>,
    title: Option<String>,
}

impl CreateTriggerBuilder {
    pub fn action(mut self, value: CreateTriggerAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn event_source_id(mut self, value: impl Into<String>) -> Self {
        self.event_source_id = Some(value.into());
        self
    }

    pub fn filters(mut self, value: impl Into<String>) -> Self {
        self.filters = Some(value.into());
        self
    }

    pub fn handler(mut self, value: TriggerHandler) -> Self {
        self.handler = Some(value);
        self
    }

    pub fn owner_type(mut self, value: TriggersOwnerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTrigger`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CreateTriggerBuilder::action)
    /// - [`event_source_id`](CreateTriggerBuilder::event_source_id)
    /// - [`handler`](CreateTriggerBuilder::handler)
    /// - [`owner_type`](CreateTriggerBuilder::owner_type)
    /// - [`title`](CreateTriggerBuilder::title)
    pub fn build(self) -> Result<CreateTrigger, BuildError> {
        Ok(CreateTrigger {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            event_source_id: self
                .event_source_id
                .ok_or_else(|| BuildError::missing_field("event_source_id"))?,
            filters: self.filters,
            handler: self
                .handler
                .ok_or_else(|| BuildError::missing_field("handler"))?,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
