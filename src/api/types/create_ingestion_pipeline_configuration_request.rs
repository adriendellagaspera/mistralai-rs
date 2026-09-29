pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateIngestionPipelineConfigurationRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_composition: Option<HashMap<String, Option<String>>>,
}

impl CreateIngestionPipelineConfigurationRequest {
    pub fn builder() -> CreateIngestionPipelineConfigurationRequestBuilder {
        <CreateIngestionPipelineConfigurationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIngestionPipelineConfigurationRequestBuilder {
    name: Option<String>,
    pipeline_composition: Option<HashMap<String, Option<String>>>,
}

impl CreateIngestionPipelineConfigurationRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pipeline_composition(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.pipeline_composition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateIngestionPipelineConfigurationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateIngestionPipelineConfigurationRequestBuilder::name)
    pub fn build(self) -> Result<CreateIngestionPipelineConfigurationRequest, BuildError> {
        Ok(CreateIngestionPipelineConfigurationRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pipeline_composition: self.pipeline_composition,
        })
    }
}
