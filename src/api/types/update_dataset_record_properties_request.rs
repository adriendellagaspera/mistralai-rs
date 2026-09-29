pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDatasetRecordPropertiesRequest {
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

impl UpdateDatasetRecordPropertiesRequest {
    pub fn builder() -> UpdateDatasetRecordPropertiesRequestBuilder {
        <UpdateDatasetRecordPropertiesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDatasetRecordPropertiesRequestBuilder {
    properties: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateDatasetRecordPropertiesRequestBuilder {
    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDatasetRecordPropertiesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`properties`](UpdateDatasetRecordPropertiesRequestBuilder::properties)
    pub fn build(self) -> Result<UpdateDatasetRecordPropertiesRequest, BuildError> {
        Ok(UpdateDatasetRecordPropertiesRequest {
            properties: self
                .properties
                .ok_or_else(|| BuildError::missing_field("properties"))?,
        })
    }
}
