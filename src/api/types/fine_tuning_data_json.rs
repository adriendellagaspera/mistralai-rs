pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FineTuningDataJson {
    /// Fine-tuning storage usage.
    #[serde(default)]
    pub storage: HashMap<String, i64>,
    /// Fine-tuning training usage.
    #[serde(default)]
    pub training: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
}

impl FineTuningDataJson {
    pub fn builder() -> FineTuningDataJsonBuilder {
        <FineTuningDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FineTuningDataJsonBuilder {
    storage: Option<HashMap<String, i64>>,
    training: Option<HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>>,
}

impl FineTuningDataJsonBuilder {
    pub fn storage(mut self, value: HashMap<String, i64>) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn training(
        mut self,
        value: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
    ) -> Self {
        self.training = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FineTuningDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`storage`](FineTuningDataJsonBuilder::storage)
    /// - [`training`](FineTuningDataJsonBuilder::training)
    pub fn build(self) -> Result<FineTuningDataJson, BuildError> {
        Ok(FineTuningDataJson {
            storage: self
                .storage
                .ok_or_else(|| BuildError::missing_field("storage"))?,
            training: self
                .training
                .ok_or_else(|| BuildError::missing_field("training"))?,
        })
    }
}
