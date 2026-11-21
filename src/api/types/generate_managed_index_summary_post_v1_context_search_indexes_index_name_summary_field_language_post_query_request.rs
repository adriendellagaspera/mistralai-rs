pub use crate::prelude::*;

/// Query parameters for generate_managed_index_summary_post_v1_context_search_indexes__index_name__summary_field__language__post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequest
{
    /// Regenerate the summary even if one already exists for this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequest {
    pub fn builder() -> GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequestBuilder {
        <GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequestBuilder
{
    force: Option<bool>,
}

impl GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequest`].
    pub fn build(self) -> Result<GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequest, BuildError> {
        Ok(GenerateManagedIndexSummaryPostV1ContextSearchIndexesIndexNameSummaryFieldLanguagePostQueryRequest {
            force: self.force,
        })
    }
}
