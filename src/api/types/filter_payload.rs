pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FilterPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterPayloadFilters>,
}

impl FilterPayload {
    pub fn builder() -> FilterPayloadBuilder {
        <FilterPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterPayloadBuilder {
    filters: Option<FilterPayloadFilters>,
}

impl FilterPayloadBuilder {
    pub fn filters(mut self, value: FilterPayloadFilters) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilterPayload`].
    pub fn build(self) -> Result<FilterPayload, BuildError> {
        Ok(FilterPayload {
            filters: self.filters,
        })
    }
}
