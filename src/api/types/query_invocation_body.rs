pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryInvocationBody {
    /// The name of the query to request
    #[serde(default)]
    pub name: String,
    /// Input data for the query, matching its schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<QueryInvocationBodyInput>,
}

impl QueryInvocationBody {
    pub fn builder() -> QueryInvocationBodyBuilder {
        <QueryInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryInvocationBodyBuilder {
    name: Option<String>,
    input: Option<QueryInvocationBodyInput>,
}

impl QueryInvocationBodyBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn input(mut self, value: QueryInvocationBodyInput) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryInvocationBodyBuilder::name)
    pub fn build(self) -> Result<QueryInvocationBody, BuildError> {
        Ok(QueryInvocationBody {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            input: self.input,
        })
    }
}
