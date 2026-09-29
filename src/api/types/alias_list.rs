pub use crate::prelude::*;

/// Presence wrapper for a set of alias labels on update RPCs. As a message field it carries presence, so callers can distinguish "leave aliases unchanged" (field omitted) from "clear all aliases" (field set, empty ``values``).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AliasList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl AliasList {
    pub fn builder() -> AliasListBuilder {
        <AliasListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AliasListBuilder {
    values: Option<Vec<String>>,
}

impl AliasListBuilder {
    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AliasList`].
    pub fn build(self) -> Result<AliasList, BuildError> {
        Ok(AliasList {
            values: self.values,
        })
    }
}
