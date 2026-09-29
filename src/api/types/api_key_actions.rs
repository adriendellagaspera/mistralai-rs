pub use crate::prelude::*;

/// Per-action availability for an API key, keyed by action name.
///
/// This is the generic, reusable shape for surfacing what a viewer may do with a resource on
/// get/list endpoints: each action maps to a value that is either available or unavailable with an
/// optional reason code. Add fields here as more actions are exposed.
///
/// Every action is opt-in: fields default to ``None`` (the action is absent), and an absent action
/// means "unavailable, with no specified reason". An action is only available when a producer
/// explicitly sets it to ``ActionAvailable``. This keeps permissions opt-in rather than opt-out, so
/// forgetting to populate an action can never accidentally expose it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiKeyActions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<ApiKeyActionsDelete>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<ApiKeyActionsRotate>,
}

impl ApiKeyActions {
    pub fn builder() -> ApiKeyActionsBuilder {
        <ApiKeyActionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyActionsBuilder {
    delete: Option<ApiKeyActionsDelete>,
    rotate: Option<ApiKeyActionsRotate>,
}

impl ApiKeyActionsBuilder {
    pub fn delete(mut self, value: ApiKeyActionsDelete) -> Self {
        self.delete = Some(value);
        self
    }

    pub fn rotate(mut self, value: ApiKeyActionsRotate) -> Self {
        self.rotate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyActions`].
    pub fn build(self) -> Result<ApiKeyActions, BuildError> {
        Ok(ApiKeyActions {
            delete: self.delete,
            rotate: self.rotate,
        })
    }
}
