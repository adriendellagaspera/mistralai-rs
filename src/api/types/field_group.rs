pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FieldGroup {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
}

impl FieldGroup {
    pub fn builder() -> FieldGroupBuilder {
        <FieldGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldGroupBuilder {
    name: Option<String>,
    label: Option<String>,
}

impl FieldGroupBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FieldGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FieldGroupBuilder::name)
    /// - [`label`](FieldGroupBuilder::label)
    pub fn build(self) -> Result<FieldGroup, BuildError> {
        Ok(FieldGroup {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
