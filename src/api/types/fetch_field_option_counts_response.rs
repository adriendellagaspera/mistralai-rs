pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FetchFieldOptionCountsResponse {
    #[serde(default)]
    pub counts: Vec<FieldOptionCountItem>,
}

impl FetchFieldOptionCountsResponse {
    pub fn builder() -> FetchFieldOptionCountsResponseBuilder {
        <FetchFieldOptionCountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchFieldOptionCountsResponseBuilder {
    counts: Option<Vec<FieldOptionCountItem>>,
}

impl FetchFieldOptionCountsResponseBuilder {
    pub fn counts(mut self, value: Vec<FieldOptionCountItem>) -> Self {
        self.counts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchFieldOptionCountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counts`](FetchFieldOptionCountsResponseBuilder::counts)
    pub fn build(self) -> Result<FetchFieldOptionCountsResponse, BuildError> {
        Ok(FetchFieldOptionCountsResponse {
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
        })
    }
}
