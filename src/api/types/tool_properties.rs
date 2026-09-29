pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ToolProperties {
    pub read_only: Option<bool>,
}

impl ToolProperties {
    pub fn builder() -> ToolPropertiesBuilder {
        <ToolPropertiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolPropertiesBuilder {
    read_only: Option<bool>,
}

impl ToolPropertiesBuilder {
    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolProperties`].
    pub fn build(self) -> Result<ToolProperties, BuildError> {
        Ok(ToolProperties {
            read_only: self.read_only,
        })
    }
}
