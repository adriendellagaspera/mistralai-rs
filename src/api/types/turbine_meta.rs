pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurbineMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ServerLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt_name: Option<String>,
}

impl TurbineMeta {
    pub fn builder() -> TurbineMetaBuilder {
        <TurbineMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurbineMetaBuilder {
    locale: Option<ServerLocale>,
    system_prompt_name: Option<String>,
}

impl TurbineMetaBuilder {
    pub fn locale(mut self, value: ServerLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn system_prompt_name(mut self, value: impl Into<String>) -> Self {
        self.system_prompt_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TurbineMeta`].
    pub fn build(self) -> Result<TurbineMeta, BuildError> {
        Ok(TurbineMeta {
            locale: self.locale,
            system_prompt_name: self.system_prompt_name,
        })
    }
}
