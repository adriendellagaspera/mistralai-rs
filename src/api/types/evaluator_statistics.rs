pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum EvaluatorStatistics {
    #[serde(rename = "numeric")]
    #[non_exhaustive]
    Numeric {
        #[serde(skip_serializing_if = "Option::is_none")]
        avg: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sample_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        std: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        values: Option<Vec<ScalarStatisticValue>>,
    },

    #[serde(rename = "categorical")]
    #[non_exhaustive]
    Categorical {
        #[serde(skip_serializing_if = "Option::is_none")]
        confusion_matrix: Option<ConfusionMatrix>,
        #[serde(default)]
        count: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_count: Option<i64>,
        #[serde(default)]
        frequencies: HashMap<String, i64>,
        #[serde(default)]
        mode: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        values: Option<Vec<ScalarStatisticValue>>,
    },

    #[serde(rename = "unsupported")]
    #[non_exhaustive]
    Unsupported {
        #[serde(default)]
        count: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl EvaluatorStatistics {
    pub fn numeric() -> Self {
        Self::Numeric {
            avg: None,
            count: None,
            max: None,
            min: None,
            sample_count: None,
            std: None,
            values: None,
        }
    }

    pub fn categorical(count: i64, frequencies: HashMap<String, i64>, mode: Vec<String>) -> Self {
        Self::Categorical {
            confusion_matrix: None,
            count,
            excluded_count: None,
            frequencies,
            mode,
            values: None,
        }
    }

    pub fn unsupported(count: i64) -> Self {
        Self::Unsupported { count }
    }

    pub fn numeric_with_avg(
        avg: f64,
        count: Option<i64>,
        max: Option<f64>,
        min: Option<f64>,
        sample_count: Option<i64>,
        std: Option<f64>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg: Some(avg),
            count,
            max,
            min,
            sample_count,
            std,
            values,
        }
    }

    pub fn numeric_with_count(
        avg: Option<f64>,
        count: i64,
        max: Option<f64>,
        min: Option<f64>,
        sample_count: Option<i64>,
        std: Option<f64>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg,
            count: Some(count),
            max,
            min,
            sample_count,
            std,
            values,
        }
    }

    pub fn numeric_with_max(
        avg: Option<f64>,
        count: Option<i64>,
        max: f64,
        min: Option<f64>,
        sample_count: Option<i64>,
        std: Option<f64>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg,
            count,
            max: Some(max),
            min,
            sample_count,
            std,
            values,
        }
    }

    pub fn numeric_with_min(
        avg: Option<f64>,
        count: Option<i64>,
        max: Option<f64>,
        min: f64,
        sample_count: Option<i64>,
        std: Option<f64>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg,
            count,
            max,
            min: Some(min),
            sample_count,
            std,
            values,
        }
    }

    pub fn numeric_with_sample_count(
        avg: Option<f64>,
        count: Option<i64>,
        max: Option<f64>,
        min: Option<f64>,
        sample_count: i64,
        std: Option<f64>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg,
            count,
            max,
            min,
            sample_count: Some(sample_count),
            std,
            values,
        }
    }

    pub fn numeric_with_std(
        avg: Option<f64>,
        count: Option<i64>,
        max: Option<f64>,
        min: Option<f64>,
        sample_count: Option<i64>,
        std: f64,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Numeric {
            avg,
            count,
            max,
            min,
            sample_count,
            std: Some(std),
            values,
        }
    }

    pub fn numeric_with_values(
        avg: Option<f64>,
        count: Option<i64>,
        max: Option<f64>,
        min: Option<f64>,
        sample_count: Option<i64>,
        std: Option<f64>,
        values: Vec<ScalarStatisticValue>,
    ) -> Self {
        Self::Numeric {
            avg,
            count,
            max,
            min,
            sample_count,
            std,
            values: Some(values),
        }
    }

    pub fn categorical_with_confusion_matrix(
        confusion_matrix: ConfusionMatrix,
        count: i64,
        excluded_count: Option<i64>,
        frequencies: HashMap<String, i64>,
        mode: Vec<String>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Categorical {
            confusion_matrix: Some(confusion_matrix),
            count,
            excluded_count,
            frequencies,
            mode,
            values,
        }
    }

    pub fn categorical_with_excluded_count(
        confusion_matrix: Option<ConfusionMatrix>,
        count: i64,
        excluded_count: i64,
        frequencies: HashMap<String, i64>,
        mode: Vec<String>,
        values: Option<Vec<ScalarStatisticValue>>,
    ) -> Self {
        Self::Categorical {
            confusion_matrix,
            count,
            excluded_count: Some(excluded_count),
            frequencies,
            mode,
            values,
        }
    }

    pub fn categorical_with_values(
        confusion_matrix: Option<ConfusionMatrix>,
        count: i64,
        excluded_count: Option<i64>,
        frequencies: HashMap<String, i64>,
        mode: Vec<String>,
        values: Vec<ScalarStatisticValue>,
    ) -> Self {
        Self::Categorical {
            confusion_matrix,
            count,
            excluded_count,
            frequencies,
            mode,
            values: Some(values),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
