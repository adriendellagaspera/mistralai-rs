pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NestedGroupRef {
    /// UUID of the group.
    #[serde(default)]
    pub uuid: String,
    /// Display name of the group.
    #[serde(default)]
    pub name: String,
}

impl NestedGroupRef {
    pub fn builder() -> NestedGroupRefBuilder {
        <NestedGroupRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NestedGroupRefBuilder {
    uuid: Option<String>,
    name: Option<String>,
}

impl NestedGroupRefBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NestedGroupRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](NestedGroupRefBuilder::uuid)
    /// - [`name`](NestedGroupRefBuilder::name)
    pub fn build(self) -> Result<NestedGroupRef, BuildError> {
        Ok(NestedGroupRef {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
