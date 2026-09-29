pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogicalExpression {
    pub r#type: LogicalExpressionType,
    #[serde(default)]
    pub expressions: Vec<Box<LogicalExpressionExpressionsItem>>,
}

impl LogicalExpression {
    pub fn builder() -> LogicalExpressionBuilder {
        <LogicalExpressionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogicalExpressionBuilder {
    r#type: Option<LogicalExpressionType>,
    expressions: Option<Vec<Box<LogicalExpressionExpressionsItem>>>,
}

impl LogicalExpressionBuilder {
    pub fn r#type(mut self, value: LogicalExpressionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn expressions(mut self, value: Vec<Box<LogicalExpressionExpressionsItem>>) -> Self {
        self.expressions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LogicalExpression`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LogicalExpressionBuilder::r#type)
    /// - [`expressions`](LogicalExpressionBuilder::expressions)
    pub fn build(self) -> Result<LogicalExpression, BuildError> {
        Ok(LogicalExpression {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            expressions: self
                .expressions
                .ok_or_else(|| BuildError::missing_field("expressions"))?,
        })
    }
}
