pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorToolLocale {
    #[serde(default)]
    pub name: HashMap<String, String>,
    #[serde(default)]
    pub description: HashMap<String, String>,
    #[serde(default)]
    pub usage_sentence: HashMap<String, String>,
}

impl ConnectorToolLocale {
    pub fn builder() -> ConnectorToolLocaleBuilder {
        <ConnectorToolLocaleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolLocaleBuilder {
    name: Option<HashMap<String, String>>,
    description: Option<HashMap<String, String>>,
    usage_sentence: Option<HashMap<String, String>>,
}

impl ConnectorToolLocaleBuilder {
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

    /// Consumes the builder and constructs a [`ConnectorToolLocale`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConnectorToolLocaleBuilder::name)
    /// - [`description`](ConnectorToolLocaleBuilder::description)
    /// - [`usage_sentence`](ConnectorToolLocaleBuilder::usage_sentence)
    pub fn build(self) -> Result<ConnectorToolLocale, BuildError> {
        Ok(ConnectorToolLocale {
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
