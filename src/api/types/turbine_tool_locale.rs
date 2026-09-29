pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurbineToolLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_sentence: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_description: Option<HashMap<String, Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_description: Option<HashMap<String, Option<String>>>,
}

impl TurbineToolLocale {
    pub fn builder() -> TurbineToolLocaleBuilder {
        <TurbineToolLocaleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurbineToolLocaleBuilder {
    name: Option<HashMap<String, Option<String>>>,
    description: Option<HashMap<String, Option<String>>>,
    usage_sentence: Option<HashMap<String, Option<String>>>,
    working_description: Option<HashMap<String, Option<String>>>,
    done_description: Option<HashMap<String, Option<String>>>,
}

impl TurbineToolLocaleBuilder {
    pub fn name(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.description = Some(value);
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

    pub fn done_description(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.done_description = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurbineToolLocale`].
    pub fn build(self) -> Result<TurbineToolLocale, BuildError> {
        Ok(TurbineToolLocale {
            name: self.name,
            description: self.description,
            usage_sentence: self.usage_sentence,
            working_description: self.working_description,
            done_description: self.done_description,
        })
    }
}
