pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurbineToolMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<TurbineToolLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<ToolType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_execution: Option<bool>,
}

impl TurbineToolMeta {
    pub fn builder() -> TurbineToolMetaBuilder {
        <TurbineToolMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurbineToolMetaBuilder {
    locale: Option<TurbineToolLocale>,
    tool_type: Option<ToolType>,
    timeout: Option<f64>,
    private_execution: Option<bool>,
}

impl TurbineToolMetaBuilder {
    pub fn locale(mut self, value: TurbineToolLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn tool_type(mut self, value: ToolType) -> Self {
        self.tool_type = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn private_execution(mut self, value: bool) -> Self {
        self.private_execution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurbineToolMeta`].
    pub fn build(self) -> Result<TurbineToolMeta, BuildError> {
        Ok(TurbineToolMeta {
            locale: self.locale,
            tool_type: self.tool_type,
            timeout: self.timeout,
            private_execution: self.private_execution,
        })
    }
}
