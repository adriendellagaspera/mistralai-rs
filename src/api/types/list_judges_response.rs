pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListJudgesResponse {
    #[serde(default)]
    pub judges: PaginatedResultJudgePreview,
}

impl ListJudgesResponse {
    pub fn builder() -> ListJudgesResponseBuilder {
        <ListJudgesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListJudgesResponseBuilder {
    judges: Option<PaginatedResultJudgePreview>,
}

impl ListJudgesResponseBuilder {
    pub fn judges(mut self, value: PaginatedResultJudgePreview) -> Self {
        self.judges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListJudgesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`judges`](ListJudgesResponseBuilder::judges)
    pub fn build(self) -> Result<ListJudgesResponse, BuildError> {
        Ok(ListJudgesResponse {
            judges: self
                .judges
                .ok_or_else(|| BuildError::missing_field("judges"))?,
        })
    }
}
