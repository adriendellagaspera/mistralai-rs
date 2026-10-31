pub use crate::prelude::*;

/// The scorer selected to build a judge pipeline configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmJudgeReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<PipelineConfigSampling>,
    #[serde(default)]
    pub slug: String,
}

impl LlmJudgeReference {
    pub fn builder() -> LlmJudgeReferenceBuilder {
        <LlmJudgeReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmJudgeReferenceBuilder {
    mapping: Option<HashMap<String, String>>,
    sampling: Option<PipelineConfigSampling>,
    slug: Option<String>,
}

impl LlmJudgeReferenceBuilder {
    pub fn mapping(mut self, value: HashMap<String, String>) -> Self {
        self.mapping = Some(value);
        self
    }

    pub fn sampling(mut self, value: PipelineConfigSampling) -> Self {
        self.sampling = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LlmJudgeReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](LlmJudgeReferenceBuilder::slug)
    pub fn build(self) -> Result<LlmJudgeReference, BuildError> {
        Ok(LlmJudgeReference {
            mapping: self.mapping,
            sampling: self.sampling,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
