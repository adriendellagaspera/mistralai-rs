pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentSearchFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ContentSearchFilter>>,
}

impl ContentSearchFilterGroup {
    pub fn builder() -> ContentSearchFilterGroupBuilder {
        <ContentSearchFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentSearchFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<ContentSearchFilter>>,
}

impl ContentSearchFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<ContentSearchFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentSearchFilterGroup`].
    pub fn build(self) -> Result<ContentSearchFilterGroup, BuildError> {
        Ok(ContentSearchFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
