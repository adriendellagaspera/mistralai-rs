pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorAuthenticationHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    #[serde(default)]
    pub name: String,
}

impl ConnectorAuthenticationHeader {
    pub fn builder() -> ConnectorAuthenticationHeaderBuilder {
        <ConnectorAuthenticationHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorAuthenticationHeaderBuilder {
    is_required: Option<bool>,
    is_secret: Option<bool>,
    name: Option<String>,
}

impl ConnectorAuthenticationHeaderBuilder {
    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorAuthenticationHeader`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConnectorAuthenticationHeaderBuilder::name)
    pub fn build(self) -> Result<ConnectorAuthenticationHeader, BuildError> {
        Ok(ConnectorAuthenticationHeader {
            is_required: self.is_required,
            is_secret: self.is_secret,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
