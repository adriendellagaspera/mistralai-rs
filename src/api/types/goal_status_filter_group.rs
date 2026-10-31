pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GoalStatusFilterGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GoalStatusFilter>>,
}

impl GoalStatusFilterGroup {
    pub fn builder() -> GoalStatusFilterGroupBuilder {
        <GoalStatusFilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoalStatusFilterGroupBuilder {
    combinator: Option<FilterCombinator>,
    filters: Option<Vec<GoalStatusFilter>>,
}

impl GoalStatusFilterGroupBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<GoalStatusFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoalStatusFilterGroup`].
    pub fn build(self) -> Result<GoalStatusFilterGroup, BuildError> {
        Ok(GoalStatusFilterGroup {
            combinator: self.combinator,
            filters: self.filters,
        })
    }
}
