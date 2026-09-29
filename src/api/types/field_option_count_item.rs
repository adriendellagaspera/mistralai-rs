pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FieldOptionCountItem {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub value: String,
}

impl FieldOptionCountItem {
    pub fn builder() -> FieldOptionCountItemBuilder {
        <FieldOptionCountItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldOptionCountItemBuilder {
    count: Option<i64>,
    value: Option<String>,
}

impl FieldOptionCountItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FieldOptionCountItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](FieldOptionCountItemBuilder::count)
    /// - [`value`](FieldOptionCountItemBuilder::value)
    pub fn build(self) -> Result<FieldOptionCountItem, BuildError> {
        Ok(FieldOptionCountItem {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
