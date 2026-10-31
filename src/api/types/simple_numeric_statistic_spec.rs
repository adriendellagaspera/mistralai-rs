pub use crate::prelude::*;

/// A scalar numeric reduction with no parameters (``avg``/``sum``/``min``/``max``/``std``/``count``).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SimpleNumericStatisticSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<GoalSpec>,
}

impl SimpleNumericStatisticSpec {
    pub fn builder() -> SimpleNumericStatisticSpecBuilder {
        <SimpleNumericStatisticSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SimpleNumericStatisticSpecBuilder {
    goal: Option<GoalSpec>,
}

impl SimpleNumericStatisticSpecBuilder {
    pub fn goal(mut self, value: GoalSpec) -> Self {
        self.goal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SimpleNumericStatisticSpec`].
    pub fn build(self) -> Result<SimpleNumericStatisticSpec, BuildError> {
        Ok(SimpleNumericStatisticSpec { goal: self.goal })
    }
}
