pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TriggerEventList {
    #[serde(default)]
    pub data: Vec<TriggerEvent>,
}

impl TriggerEventList {
    pub fn builder() -> TriggerEventListBuilder {
        <TriggerEventListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerEventListBuilder {
    data: Option<Vec<TriggerEvent>>,
}

impl TriggerEventListBuilder {
    pub fn data(mut self, value: Vec<TriggerEvent>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TriggerEventList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](TriggerEventListBuilder::data)
    pub fn build(self) -> Result<TriggerEventList, BuildError> {
        Ok(TriggerEventList {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
