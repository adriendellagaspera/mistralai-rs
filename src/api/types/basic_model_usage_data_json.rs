pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BasicModelUsageDataJson {
    /// Usage data grouped by model.
    #[serde(default)]
    pub models: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
}

impl BasicModelUsageDataJson {
    pub fn builder() -> BasicModelUsageDataJsonBuilder {
        <BasicModelUsageDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicModelUsageDataJsonBuilder {
    models: Option<HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>>,
}

impl BasicModelUsageDataJsonBuilder {
    pub fn models(
        mut self,
        value: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
    ) -> Self {
        self.models = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BasicModelUsageDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`models`](BasicModelUsageDataJsonBuilder::models)
    pub fn build(self) -> Result<BasicModelUsageDataJson, BuildError> {
        Ok(BasicModelUsageDataJson {
            models: self
                .models
                .ok_or_else(|| BuildError::missing_field("models"))?,
        })
    }
}
