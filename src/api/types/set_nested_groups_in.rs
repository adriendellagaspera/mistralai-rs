pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetNestedGroupsIn {
    /// Full replacement list of UUIDs of groups to nest directly inside this group.
    #[serde(default)]
    pub child_group_uuids: Vec<String>,
}

impl SetNestedGroupsIn {
    pub fn builder() -> SetNestedGroupsInBuilder {
        <SetNestedGroupsInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetNestedGroupsInBuilder {
    child_group_uuids: Option<Vec<String>>,
}

impl SetNestedGroupsInBuilder {
    pub fn child_group_uuids(mut self, value: Vec<String>) -> Self {
        self.child_group_uuids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetNestedGroupsIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child_group_uuids`](SetNestedGroupsInBuilder::child_group_uuids)
    pub fn build(self) -> Result<SetNestedGroupsIn, BuildError> {
        Ok(SetNestedGroupsIn {
            child_group_uuids: self
                .child_group_uuids
                .ok_or_else(|| BuildError::missing_field("child_group_uuids"))?,
        })
    }
}
