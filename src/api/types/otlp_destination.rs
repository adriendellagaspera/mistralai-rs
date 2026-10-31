pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OtlpDestination {
    #[serde(default)]
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, PipelineConfigHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default)]
    pub protocol: String,
}

impl OtlpDestination {
    pub fn builder() -> OtlpDestinationBuilder {
        <OtlpDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OtlpDestinationBuilder {
    endpoint: Option<String>,
    headers: Option<HashMap<String, PipelineConfigHeader>>,
    insecure: Option<bool>,
    protocol: Option<String>,
}

impl OtlpDestinationBuilder {
    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, PipelineConfigHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn insecure(mut self, value: bool) -> Self {
        self.insecure = Some(value);
        self
    }

    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OtlpDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint`](OtlpDestinationBuilder::endpoint)
    /// - [`protocol`](OtlpDestinationBuilder::protocol)
    pub fn build(self) -> Result<OtlpDestination, BuildError> {
        Ok(OtlpDestination {
            endpoint: self
                .endpoint
                .ok_or_else(|| BuildError::missing_field("endpoint"))?,
            headers: self.headers,
            insecure: self.insecure,
            protocol: self
                .protocol
                .ok_or_else(|| BuildError::missing_field("protocol"))?,
        })
    }
}
