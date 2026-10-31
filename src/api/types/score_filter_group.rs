pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScoreFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ScoreFilter>>,
}

impl ScoreFilterGroup {
    pub fn builder() -> ScoreFilterGroupBuilder {
        <ScoreFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<ScoreFilter>>,
}

impl ScoreFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<ScoreFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreFilterGroup`].
    pub fn build(self) -> Result<ScoreFilterGroup, BuildError> {
        Ok(ScoreFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
