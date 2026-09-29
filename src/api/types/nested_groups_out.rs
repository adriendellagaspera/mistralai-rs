pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NestedGroupsOut {
    /// Groups directly nested inside this group (its direct children).
    #[serde(default)]
    pub children: Vec<NestedGroupRef>,
}

impl NestedGroupsOut {
    pub fn builder() -> NestedGroupsOutBuilder {
        <NestedGroupsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NestedGroupsOutBuilder {
    children: Option<Vec<NestedGroupRef>>,
}

impl NestedGroupsOutBuilder {
    pub fn children(mut self, value: Vec<NestedGroupRef>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NestedGroupsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children`](NestedGroupsOutBuilder::children)
    pub fn build(self) -> Result<NestedGroupsOut, BuildError> {
        Ok(NestedGroupsOut {
            children: self
                .children
                .ok_or_else(|| BuildError::missing_field("children"))?,
        })
    }
}
