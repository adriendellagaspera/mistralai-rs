pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MetadataFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MetadataFilter>>,
}

impl MetadataFilterGroup {
    pub fn builder() -> MetadataFilterGroupBuilder {
        <MetadataFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetadataFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<MetadataFilter>>,
}

impl MetadataFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<MetadataFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetadataFilterGroup`].
    pub fn build(self) -> Result<MetadataFilterGroup, BuildError> {
        Ok(MetadataFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
