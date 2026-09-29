pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPromptVersionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PromptVersion>>,
}

impl ListPromptVersionsResponse {
    pub fn builder() -> ListPromptVersionsResponseBuilder {
        <ListPromptVersionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPromptVersionsResponseBuilder {
    data: Option<Vec<PromptVersion>>,
}

impl ListPromptVersionsResponseBuilder {
    pub fn data(mut self, value: Vec<PromptVersion>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPromptVersionsResponse`].
    pub fn build(self) -> Result<ListPromptVersionsResponse, BuildError> {
        Ok(ListPromptVersionsResponse { data: self.data })
    }
}
