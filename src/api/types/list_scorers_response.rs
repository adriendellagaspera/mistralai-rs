pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListScorersResponse {
    #[serde(default)]
    pub scorers: PaginatedResultScorerSummary,
}

impl ListScorersResponse {
    pub fn builder() -> ListScorersResponseBuilder {
        <ListScorersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListScorersResponseBuilder {
    scorers: Option<PaginatedResultScorerSummary>,
}

impl ListScorersResponseBuilder {
    pub fn scorers(mut self, value: PaginatedResultScorerSummary) -> Self {
        self.scorers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListScorersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scorers`](ListScorersResponseBuilder::scorers)
    pub fn build(self) -> Result<ListScorersResponse, BuildError> {
        Ok(ListScorersResponse {
            scorers: self
                .scorers
                .ok_or_else(|| BuildError::missing_field("scorers"))?,
        })
    }
}
