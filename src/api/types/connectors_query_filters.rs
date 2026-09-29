pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorsQueryFilters {
    /// Filter for active connectors for a given user, workspace and organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl ConnectorsQueryFilters {
    pub fn builder() -> ConnectorsQueryFiltersBuilder {
        <ConnectorsQueryFiltersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorsQueryFiltersBuilder {
    active: Option<bool>,
}

impl ConnectorsQueryFiltersBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorsQueryFilters`].
    pub fn build(self) -> Result<ConnectorsQueryFilters, BuildError> {
        Ok(ConnectorsQueryFilters {
            active: self.active,
        })
    }
}
