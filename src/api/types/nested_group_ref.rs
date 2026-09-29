pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NestedGroupRef {
    /// Display name of the group.
    #[serde(default)]
    pub name: String,
    /// UUID of the group.
    #[serde(default)]
    pub uuid: String,
}

impl NestedGroupRef {
    pub fn builder() -> NestedGroupRefBuilder {
        <NestedGroupRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NestedGroupRefBuilder {
    name: Option<String>,
    uuid: Option<String>,
}

impl NestedGroupRefBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NestedGroupRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](NestedGroupRefBuilder::name)
    /// - [`uuid`](NestedGroupRefBuilder::uuid)
    pub fn build(self) -> Result<NestedGroupRef, BuildError> {
        Ok(NestedGroupRef {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
