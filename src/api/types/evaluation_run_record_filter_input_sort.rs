pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum EvaluationRunRecordFilterInputSort {
    #[serde(rename = "field")]
    #[non_exhaustive]
    Field {
        #[serde(skip_serializing_if = "Option::is_none")]
        direction: Option<SortDirection>,
        field: RecordSortField,
    },

    #[serde(rename = "score")]
    #[non_exhaustive]
    Score {
        #[serde(skip_serializing_if = "Option::is_none")]
        direction: Option<SortDirection>,
        #[serde(default)]
        evaluator_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metric: Option<NumericScoreMetric>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl EvaluationRunRecordFilterInputSort {
    pub fn field(field: RecordSortField) -> Self {
        Self::Field {
            direction: None,
            field,
        }
    }

    pub fn score(evaluator_name: String) -> Self {
        Self::Score {
            direction: None,
            evaluator_name,
            metric: None,
        }
    }

    pub fn field_with_direction(direction: SortDirection, field: RecordSortField) -> Self {
        Self::Field {
            direction: Some(direction),
            field,
        }
    }

    pub fn score_with_direction(
        direction: SortDirection,
        evaluator_name: String,
        metric: Option<NumericScoreMetric>,
    ) -> Self {
        Self::Score {
            direction: Some(direction),
            evaluator_name,
            metric,
        }
    }

    pub fn score_with_metric(
        direction: Option<SortDirection>,
        evaluator_name: String,
        metric: NumericScoreMetric,
    ) -> Self {
        Self::Score {
            direction,
            evaluator_name,
            metric: Some(metric),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
