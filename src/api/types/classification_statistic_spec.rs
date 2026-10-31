pub use crate::prelude::*;

/// A classification metric (``precision``/``recall``/``f1``) over ``(expected, predicted)`` pairs.
///
/// The predicted label is the score ``value``; the expected label is read from the score ``metadata``
/// under ``expected_key`` (default ``"expected"``). Provide **exactly one** of ``positive_label`` (a
/// binary metric scored against that one class) or ``average`` (a multi-class averaging scheme).
///
/// ``labels`` pins the class universe so filtered views keep a stable, comparable denominator.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClassificationStatisticSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<ClassificationStatisticSpecAverage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<GoalSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_label: Option<String>,
}

impl ClassificationStatisticSpec {
    pub fn builder() -> ClassificationStatisticSpecBuilder {
        <ClassificationStatisticSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationStatisticSpecBuilder {
    average: Option<ClassificationStatisticSpecAverage>,
    expected_key: Option<String>,
    goal: Option<GoalSpec>,
    labels: Option<Vec<String>>,
    positive_label: Option<String>,
}

impl ClassificationStatisticSpecBuilder {
    pub fn average(mut self, value: ClassificationStatisticSpecAverage) -> Self {
        self.average = Some(value);
        self
    }

    pub fn expected_key(mut self, value: impl Into<String>) -> Self {
        self.expected_key = Some(value.into());
        self
    }

    pub fn goal(mut self, value: GoalSpec) -> Self {
        self.goal = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn positive_label(mut self, value: impl Into<String>) -> Self {
        self.positive_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClassificationStatisticSpec`].
    pub fn build(self) -> Result<ClassificationStatisticSpec, BuildError> {
        Ok(ClassificationStatisticSpec {
            average: self.average,
            expected_key: self.expected_key,
            goal: self.goal,
            labels: self.labels,
            positive_label: self.positive_label,
        })
    }
}
