pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogicalExpression {
    #[serde(default)]
    pub expressions: Vec<Box<LogicalExpressionExpressionsItem>>,
    pub r#type: LogicalExpressionType,
}

impl LogicalExpression {
    pub fn builder() -> LogicalExpressionBuilder {
        <LogicalExpressionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogicalExpressionBuilder {
    expressions: Option<Vec<Box<LogicalExpressionExpressionsItem>>>,
    r#type: Option<LogicalExpressionType>,
}

impl LogicalExpressionBuilder {
    pub fn expressions(mut self, value: Vec<Box<LogicalExpressionExpressionsItem>>) -> Self {
        self.expressions = Some(value);
        self
    }

    pub fn r#type(mut self, value: LogicalExpressionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LogicalExpression`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expressions`](LogicalExpressionBuilder::expressions)
    /// - [`r#type`](LogicalExpressionBuilder::r#type)
    pub fn build(self) -> Result<LogicalExpression, BuildError> {
        Ok(LogicalExpression {
            expressions: self
                .expressions
                .ok_or_else(|| BuildError::missing_field("expressions"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
