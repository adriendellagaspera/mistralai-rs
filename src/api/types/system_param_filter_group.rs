pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemParamFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SystemParamFilter>>,
}

impl SystemParamFilterGroup {
    pub fn builder() -> SystemParamFilterGroupBuilder {
        <SystemParamFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemParamFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<SystemParamFilter>>,
}

impl SystemParamFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<SystemParamFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemParamFilterGroup`].
    pub fn build(self) -> Result<SystemParamFilterGroup, BuildError> {
        Ok(SystemParamFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
