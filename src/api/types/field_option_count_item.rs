pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FieldOptionCountItem {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub count: i64,
}

impl FieldOptionCountItem {
    pub fn builder() -> FieldOptionCountItemBuilder {
        <FieldOptionCountItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldOptionCountItemBuilder {
    value: Option<String>,
    count: Option<i64>,
}

impl FieldOptionCountItemBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FieldOptionCountItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](FieldOptionCountItemBuilder::value)
    /// - [`count`](FieldOptionCountItemBuilder::count)
    pub fn build(self) -> Result<FieldOptionCountItem, BuildError> {
        Ok(FieldOptionCountItem {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
