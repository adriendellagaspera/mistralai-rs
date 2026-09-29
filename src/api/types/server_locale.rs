pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServerLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_description: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_sentence: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_description: Option<HashMap<String, Option<String>>>,
}

impl ServerLocale {
    pub fn builder() -> ServerLocaleBuilder {
        <ServerLocaleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServerLocaleBuilder {
    description: Option<HashMap<String, Option<String>>>,
    done_description: Option<HashMap<String, Option<String>>>,
    name: Option<HashMap<String, Option<String>>>,
    usage_sentence: Option<HashMap<String, Option<String>>>,
    working_description: Option<HashMap<String, Option<String>>>,
}

impl ServerLocaleBuilder {
    pub fn description(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn done_description(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.done_description = Some(value);
        self
    }

    pub fn name(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn usage_sentence(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.usage_sentence = Some(value);
        self
    }

    pub fn working_description(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.working_description = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServerLocale`].
    pub fn build(self) -> Result<ServerLocale, BuildError> {
        Ok(ServerLocale {
            description: self.description,
            done_description: self.done_description,
            name: self.name,
            usage_sentence: self.usage_sentence,
            working_description: self.working_description,
        })
    }
}
