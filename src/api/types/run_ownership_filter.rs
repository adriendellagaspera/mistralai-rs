pub use crate::prelude::*;

/// OR-able group isolating the "runs I care about" default (created by me OR in my starred projects).
///
/// Kept separate from the plain generic filters so it can OR internally while still AND-ing with
/// everything else. The UI drives it via two sugar chips: "Created by me" writes `creator_ids`, and
/// "Starred projects" sets `starred_by_me`. Dropping a chip clears its term and the OR collapses to the
/// remaining one; dropping both leaves the group empty (no ownership scoping).
///
/// `starred_by_me` is resolved server-side against the caller's favorites, so the client never has to
/// page through projects to collect starred ids — and a caller with zero starred projects correctly
/// matches no runs (rather than falling through to the whole workspace). `project_ids` remains for
/// explicit, caller-supplied project scoping.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunOwnershipFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred_by_me: Option<bool>,
}

impl RunOwnershipFilter {
    pub fn builder() -> RunOwnershipFilterBuilder {
        <RunOwnershipFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunOwnershipFilterBuilder {
    combinator: Option<FilterCombinator>,
    creator_ids: Option<Vec<String>>,
    project_ids: Option<Vec<String>>,
    starred_by_me: Option<bool>,
}

impl RunOwnershipFilterBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn creator_ids(mut self, value: Vec<String>) -> Self {
        self.creator_ids = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    pub fn starred_by_me(mut self, value: bool) -> Self {
        self.starred_by_me = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunOwnershipFilter`].
    pub fn build(self) -> Result<RunOwnershipFilter, BuildError> {
        Ok(RunOwnershipFilter {
            combinator: self.combinator,
            creator_ids: self.creator_ids,
            project_ids: self.project_ids,
            starred_by_me: self.starred_by_me,
        })
    }
}
