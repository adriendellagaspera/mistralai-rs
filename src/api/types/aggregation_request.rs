pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    pub metric: MetricDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderByClause>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_dimension: Option<TimeDimension>,
}

impl AggregationRequest {
    pub fn builder() -> AggregationRequestBuilder {
        <AggregationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationRequestBuilder {
    dimensions: Option<Vec<String>>,
    limit: Option<i64>,
    metric: Option<MetricDefinition>,
    order_by: Option<Vec<OrderByClause>>,
    search_expression: Option<String>,
    time_dimension: Option<TimeDimension>,
}

impl AggregationRequestBuilder {
    pub fn dimensions(mut self, value: Vec<String>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn metric(mut self, value: MetricDefinition) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn order_by(mut self, value: Vec<OrderByClause>) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn search_expression(mut self, value: impl Into<String>) -> Self {
        self.search_expression = Some(value.into());
        self
    }

    pub fn time_dimension(mut self, value: TimeDimension) -> Self {
        self.time_dimension = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metric`](AggregationRequestBuilder::metric)
    pub fn build(self) -> Result<AggregationRequest, BuildError> {
        Ok(AggregationRequest {
            dimensions: self.dimensions,
            limit: self.limit,
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            order_by: self.order_by,
            search_expression: self.search_expression,
            time_dimension: self.time_dimension,
        })
    }
}
