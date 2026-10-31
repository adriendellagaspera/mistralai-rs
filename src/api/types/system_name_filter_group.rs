pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemNameFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SystemNameFilter>>,
}

impl SystemNameFilterGroup {
    pub fn builder() -> SystemNameFilterGroupBuilder {
        <SystemNameFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemNameFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<SystemNameFilter>>,
}

impl SystemNameFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<SystemNameFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemNameFilterGroup`].
    pub fn build(self) -> Result<SystemNameFilterGroup, BuildError> {
        Ok(SystemNameFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
