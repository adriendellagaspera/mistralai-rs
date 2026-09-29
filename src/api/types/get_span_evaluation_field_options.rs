pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanEvaluationFieldOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl GetSpanEvaluationFieldOptions {
    pub fn builder() -> GetSpanEvaluationFieldOptionsBuilder {
        <GetSpanEvaluationFieldOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanEvaluationFieldOptionsBuilder {
    options: Option<Vec<String>>,
}

impl GetSpanEvaluationFieldOptionsBuilder {
    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanEvaluationFieldOptions`].
    pub fn build(self) -> Result<GetSpanEvaluationFieldOptions, BuildError> {
        Ok(GetSpanEvaluationFieldOptions {
            options: self.options,
        })
    }
}
