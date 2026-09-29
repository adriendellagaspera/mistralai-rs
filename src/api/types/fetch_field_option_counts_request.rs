pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FetchFieldOptionCountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_params: Option<FilterPayload>,
}

impl FetchFieldOptionCountsRequest {
    pub fn builder() -> FetchFieldOptionCountsRequestBuilder {
        <FetchFieldOptionCountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchFieldOptionCountsRequestBuilder {
    filter_params: Option<FilterPayload>,
}

impl FetchFieldOptionCountsRequestBuilder {
    pub fn filter_params(mut self, value: FilterPayload) -> Self {
        self.filter_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchFieldOptionCountsRequest`].
    pub fn build(self) -> Result<FetchFieldOptionCountsRequest, BuildError> {
        Ok(FetchFieldOptionCountsRequest {
            filter_params: self.filter_params,
        })
    }
}
