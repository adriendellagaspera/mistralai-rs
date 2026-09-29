pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorLocale {
    #[serde(default)]
    pub name: HashMap<String, String>,
    #[serde(default)]
    pub description: HashMap<String, String>,
    #[serde(default)]
    pub usage_sentence: HashMap<String, String>,
}

impl ConnectorLocale {
    pub fn builder() -> ConnectorLocaleBuilder {
        <ConnectorLocaleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorLocaleBuilder {
    name: Option<HashMap<String, String>>,
    description: Option<HashMap<String, String>>,
    usage_sentence: Option<HashMap<String, String>>,
}

impl ConnectorLocaleBuilder {
    pub fn name(mut self, value: HashMap<String, String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: HashMap<String, String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn usage_sentence(mut self, value: HashMap<String, String>) -> Self {
        self.usage_sentence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorLocale`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConnectorLocaleBuilder::name)
    /// - [`description`](ConnectorLocaleBuilder::description)
    /// - [`usage_sentence`](ConnectorLocaleBuilder::usage_sentence)
    pub fn build(self) -> Result<ConnectorLocale, BuildError> {
        Ok(ConnectorLocale {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            usage_sentence: self
                .usage_sentence
                .ok_or_else(|| BuildError::missing_field("usage_sentence"))?,
        })
    }
}
