pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicConnectionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConnectionConfigType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
}

impl PublicConnectionConfig {
    pub fn builder() -> PublicConnectionConfigBuilder {
        <PublicConnectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicConnectionConfigBuilder {
    r#type: Option<ConnectionConfigType>,
    base_url: Option<String>,
    headers: Option<HashMap<String, Option<String>>>,
    signed: Option<bool>,
}

impl PublicConnectionConfigBuilder {
    pub fn r#type(mut self, value: ConnectionConfigType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn signed(mut self, value: bool) -> Self {
        self.signed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicConnectionConfig`].
    pub fn build(self) -> Result<PublicConnectionConfig, BuildError> {
        Ok(PublicConnectionConfig {
            r#type: self.r#type,
            base_url: self.base_url,
            headers: self.headers,
            signed: self.signed,
        })
    }
}
