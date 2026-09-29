pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDatasetRecordRequest {
    #[serde(default)]
    pub payload: DatasetRecordPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
}

impl CreateDatasetRecordRequest {
    pub fn builder() -> CreateDatasetRecordRequestBuilder {
        <CreateDatasetRecordRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDatasetRecordRequestBuilder {
    payload: Option<DatasetRecordPayload>,
    properties: Option<HashMap<String, serde_json::Value>>,
}

impl CreateDatasetRecordRequestBuilder {
    pub fn payload(mut self, value: DatasetRecordPayload) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDatasetRecordRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payload`](CreateDatasetRecordRequestBuilder::payload)
    pub fn build(self) -> Result<CreateDatasetRecordRequest, BuildError> {
        Ok(CreateDatasetRecordRequest {
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
            properties: self.properties,
        })
    }
}
