pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum EvaluatorV2StatisticsItem {
    #[serde(rename = "avg")]
    #[non_exhaustive]
    Avg {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    #[serde(rename = "count")]
    #[non_exhaustive]
    Count {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    #[serde(rename = "f1")]
    #[non_exhaustive]
    F1 {
        #[serde(flatten)]
        data: ClassificationStatisticSpec,
    },

    #[serde(rename = "max")]
    #[non_exhaustive]
    Max {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    #[serde(rename = "min")]
    #[non_exhaustive]
    Min {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    #[serde(rename = "percentile")]
    #[non_exhaustive]
    Percentile {
        #[serde(skip_serializing_if = "Option::is_none")]
        goal: Option<GoalSpec>,
        percentile: PercentileStatisticSpecPercentile,
    },

    #[serde(rename = "precision")]
    #[non_exhaustive]
    Precision {
        #[serde(flatten)]
        data: ClassificationStatisticSpec,
    },

    #[serde(rename = "recall")]
    #[non_exhaustive]
    Recall {
        #[serde(flatten)]
        data: ClassificationStatisticSpec,
    },

    #[serde(rename = "std")]
    #[non_exhaustive]
    Std {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    #[serde(rename = "sum")]
    #[non_exhaustive]
    Sum {
        #[serde(flatten)]
        data: SimpleNumericStatisticSpec,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl EvaluatorV2StatisticsItem {
    pub fn avg(data: SimpleNumericStatisticSpec) -> Self {
        Self::Avg { data }
    }

    pub fn count(data: SimpleNumericStatisticSpec) -> Self {
        Self::Count { data }
    }

    pub fn f1(data: ClassificationStatisticSpec) -> Self {
        Self::F1 { data }
    }

    pub fn max(data: SimpleNumericStatisticSpec) -> Self {
        Self::Max { data }
    }

    pub fn min(data: SimpleNumericStatisticSpec) -> Self {
        Self::Min { data }
    }

    pub fn percentile(percentile: PercentileStatisticSpecPercentile) -> Self {
        Self::Percentile {
            goal: None,
            percentile,
        }
    }

    pub fn precision(data: ClassificationStatisticSpec) -> Self {
        Self::Precision { data }
    }

    pub fn recall(data: ClassificationStatisticSpec) -> Self {
        Self::Recall { data }
    }

    pub fn std(data: SimpleNumericStatisticSpec) -> Self {
        Self::Std { data }
    }

    pub fn sum(data: SimpleNumericStatisticSpec) -> Self {
        Self::Sum { data }
    }

    pub fn percentile_with_goal(
        goal: GoalSpec,
        percentile: PercentileStatisticSpecPercentile,
    ) -> Self {
        Self::Percentile {
            goal: Some(goal),
            percentile,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
