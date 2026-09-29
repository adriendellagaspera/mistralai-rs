pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FineTuningDataJson {
    /// Fine-tuning training usage.
    #[serde(default)]
    pub training: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
    /// Fine-tuning storage usage.
    #[serde(default)]
    pub storage: HashMap<String, i64>,
}

impl FineTuningDataJson {
    pub fn builder() -> FineTuningDataJsonBuilder {
        <FineTuningDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FineTuningDataJsonBuilder {
    training: Option<HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>>,
    storage: Option<HashMap<String, i64>>,
}

impl FineTuningDataJsonBuilder {
    pub fn training(
        mut self,
        value: HashMap<String, HashMap<String, Vec<HashMap<String, serde_json::Value>>>>,
    ) -> Self {
        self.training = Some(value);
        self
    }

    pub fn storage(mut self, value: HashMap<String, i64>) -> Self {
        self.storage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FineTuningDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`training`](FineTuningDataJsonBuilder::training)
    /// - [`storage`](FineTuningDataJsonBuilder::storage)
    pub fn build(self) -> Result<FineTuningDataJson, BuildError> {
        Ok(FineTuningDataJson {
            training: self
                .training
                .ok_or_else(|| BuildError::missing_field("training"))?,
            storage: self
                .storage
                .ok_or_else(|| BuildError::missing_field("storage"))?,
        })
    }
}
