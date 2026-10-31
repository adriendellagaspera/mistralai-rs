pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentSearchFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<ContentSearchOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    pub value: String,
}

impl ContentSearchFilter {
    pub fn builder() -> ContentSearchFilterBuilder {
        <ContentSearchFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentSearchFilterBuilder {
    operator: Option<ContentSearchOperator>,
    path: Option<String>,
    value: Option<String>,
}

impl ContentSearchFilterBuilder {
    pub fn operator(mut self, value: ContentSearchOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContentSearchFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ContentSearchFilterBuilder::value)
    pub fn build(self) -> Result<ContentSearchFilter, BuildError> {
        Ok(ContentSearchFilter {
            operator: self.operator,
            path: self.path,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
