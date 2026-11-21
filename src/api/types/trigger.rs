pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Trigger {
    pub action: TriggerAction,
    #[serde(default)]
    pub creator_id: String,
    #[serde(default)]
    pub customer_id: String,
    pub event_source: Webhook,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    pub handler: TriggerHandler,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub organization_id: String,
    pub owner_type: TriggersOwnerType,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl Trigger {
    pub fn builder() -> TriggerBuilder {
        <TriggerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerBuilder {
    action: Option<TriggerAction>,
    creator_id: Option<String>,
    customer_id: Option<String>,
    event_source: Option<Webhook>,
    filters: Option<String>,
    handler: Option<TriggerHandler>,
    id: Option<String>,
    organization_id: Option<String>,
    owner_type: Option<TriggersOwnerType>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl TriggerBuilder {
    pub fn action(mut self, value: TriggerAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn event_source(mut self, value: Webhook) -> Self {
        self.event_source = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
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

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Trigger`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](TriggerBuilder::action)
    /// - [`creator_id`](TriggerBuilder::creator_id)
    /// - [`customer_id`](TriggerBuilder::customer_id)
    /// - [`event_source`](TriggerBuilder::event_source)
    /// - [`handler`](TriggerBuilder::handler)
    /// - [`id`](TriggerBuilder::id)
    /// - [`organization_id`](TriggerBuilder::organization_id)
    /// - [`owner_type`](TriggerBuilder::owner_type)
    /// - [`title`](TriggerBuilder::title)
    /// - [`workspace_id`](TriggerBuilder::workspace_id)
    pub fn build(self) -> Result<Trigger, BuildError> {
        Ok(Trigger {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            event_source: self
                .event_source
                .ok_or_else(|| BuildError::missing_field("event_source"))?,
            filters: self.filters,
            handler: self
                .handler
                .ok_or_else(|| BuildError::missing_field("handler"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
