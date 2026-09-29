pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorAuthenticationHeader {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}

impl ConnectorAuthenticationHeader {
    pub fn builder() -> ConnectorAuthenticationHeaderBuilder {
        <ConnectorAuthenticationHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorAuthenticationHeaderBuilder {
    name: Option<String>,
    is_required: Option<bool>,
    is_secret: Option<bool>,
}

impl ConnectorAuthenticationHeaderBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorAuthenticationHeader`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConnectorAuthenticationHeaderBuilder::name)
    pub fn build(self) -> Result<ConnectorAuthenticationHeader, BuildError> {
        Ok(ConnectorAuthenticationHeader {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_required: self.is_required,
            is_secret: self.is_secret,
        })
    }
}
