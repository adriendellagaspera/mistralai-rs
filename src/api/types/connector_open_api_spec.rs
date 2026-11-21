pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorOpenApiSpec {
    /// Where the spec came from. Null when unknown, e.g. a file uploaded without a name, or hidden, e.g. on Mistral connectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<OpenApiSpecSource>,
}

impl ConnectorOpenApiSpec {
    pub fn builder() -> ConnectorOpenApiSpecBuilder {
        <ConnectorOpenApiSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorOpenApiSpecBuilder {
    source: Option<OpenApiSpecSource>,
}

impl ConnectorOpenApiSpecBuilder {
    pub fn source(mut self, value: OpenApiSpecSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorOpenApiSpec`].
    pub fn build(self) -> Result<ConnectorOpenApiSpec, BuildError> {
        Ok(ConnectorOpenApiSpec {
            source: self.source,
        })
    }
}
