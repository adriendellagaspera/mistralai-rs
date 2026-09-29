pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurbineMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ServerLocale>,
}

impl TurbineMeta {
    pub fn builder() -> TurbineMetaBuilder {
        <TurbineMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurbineMetaBuilder {
    system_prompt_name: Option<String>,
    locale: Option<ServerLocale>,
}

impl TurbineMetaBuilder {
    pub fn system_prompt_name(mut self, value: impl Into<String>) -> Self {
        self.system_prompt_name = Some(value.into());
        self
    }

    pub fn locale(mut self, value: ServerLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurbineMeta`].
    pub fn build(self) -> Result<TurbineMeta, BuildError> {
        Ok(TurbineMeta {
            system_prompt_name: self.system_prompt_name,
            locale: self.locale,
        })
    }
}
