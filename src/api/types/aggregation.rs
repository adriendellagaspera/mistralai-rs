pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Aggregation {
    #[serde(default)]
    pub data: Vec<AggregationRow>,
    #[serde(default)]
    pub meta: AggregationMeta,
}

impl Aggregation {
    pub fn builder() -> AggregationBuilder {
        <AggregationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationBuilder {
    data: Option<Vec<AggregationRow>>,
    meta: Option<AggregationMeta>,
}

impl AggregationBuilder {
    pub fn data(mut self, value: Vec<AggregationRow>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: AggregationMeta) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Aggregation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AggregationBuilder::data)
    /// - [`meta`](AggregationBuilder::meta)
    pub fn build(self) -> Result<Aggregation, BuildError> {
        Ok(Aggregation {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            meta: self.meta.ok_or_else(|| BuildError::missing_field("meta"))?,
        })
    }
}
