pub use crate::prelude::*;

/// The viewer may perform the action on the resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActionAvailable {}

impl ActionAvailable {
    pub fn builder() -> ActionAvailableBuilder {
        <ActionAvailableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActionAvailableBuilder {}

impl ActionAvailableBuilder {
    /// Consumes the builder and constructs a [`ActionAvailable`].
    pub fn build(self) -> Result<ActionAvailable, BuildError> {
        Ok(ActionAvailable {})
    }
}
