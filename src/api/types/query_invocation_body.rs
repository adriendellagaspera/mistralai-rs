pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryInvocationBody {
    /// Input data for the query, matching its schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<QueryInvocationBodyInput>,
    /// The name of the query to request
    #[serde(default)]
    pub name: String,
}

impl QueryInvocationBody {
    pub fn builder() -> QueryInvocationBodyBuilder {
        <QueryInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryInvocationBodyBuilder {
    input: Option<QueryInvocationBodyInput>,
    name: Option<String>,
}

impl QueryInvocationBodyBuilder {
    pub fn input(mut self, value: QueryInvocationBodyInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryInvocationBodyBuilder::name)
    pub fn build(self) -> Result<QueryInvocationBody, BuildError> {
        Ok(QueryInvocationBody {
            input: self.input,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
