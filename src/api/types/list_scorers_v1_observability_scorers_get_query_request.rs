pub use crate::prelude::*;

/// Query parameters for list_scorers_v1_observability_scorers_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListScorersV1ObservabilityScorersGetQueryRequest {
    /// Only return scorers created by the current user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_me: Option<bool>,
    /// Only return out-of-the-box scorers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ootb: Option<bool>,
    /// Case-insensitive filter on scorer slug, name, or description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListScorersV1ObservabilityScorersGetQueryRequest {
    pub fn builder() -> ListScorersV1ObservabilityScorersGetQueryRequestBuilder {
        <ListScorersV1ObservabilityScorersGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListScorersV1ObservabilityScorersGetQueryRequestBuilder {
    created_by_me: Option<bool>,
    ootb: Option<bool>,
    q: Option<String>,
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListScorersV1ObservabilityScorersGetQueryRequestBuilder {
    pub fn created_by_me(mut self, value: bool) -> Self {
        self.created_by_me = Some(value);
        self
    }

    pub fn ootb(mut self, value: bool) -> Self {
        self.ootb = Some(value);
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListScorersV1ObservabilityScorersGetQueryRequest`].
    pub fn build(self) -> Result<ListScorersV1ObservabilityScorersGetQueryRequest, BuildError> {
        Ok(ListScorersV1ObservabilityScorersGetQueryRequest {
            created_by_me: self.created_by_me,
            ootb: self.ootb,
            q: self.q,
            page_size: self.page_size,
            page: self.page,
        })
    }
}
