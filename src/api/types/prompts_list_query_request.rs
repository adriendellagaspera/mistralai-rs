pub use crate::prelude::*;

/// Query parameters for prompts_list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptsListQueryRequest {
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// Defaults to created_at when omitted.
    #[serde(rename = "sort.field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListSortField>,
    /// Defaults to descending for timestamp fields and ascending for text fields.
    #[serde(rename = "sort.direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction_legacy: Option<ListSortDirection>,
    /// REST-friendly alias for sort.field. Supported values: created_at, last_modified_at, name, title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// REST-friendly alias for sort.direction. Supported values: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl PromptsListQueryRequest {
    pub fn builder() -> PromptsListQueryRequestBuilder {
        <PromptsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptsListQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    alias: Option<String>,
    fields: Option<Vec<Option<String>>>,
    sort_field: Option<ListSortField>,
    sort_direction_legacy: Option<ListSortDirection>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
}

impl PromptsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn sort_field(mut self, value: ListSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_direction_legacy(mut self, value: ListSortDirection) -> Self {
        self.sort_direction_legacy = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: impl Into<String>) -> Self {
        self.sort_direction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromptsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](PromptsListQueryRequestBuilder::fields)
    pub fn build(self) -> Result<PromptsListQueryRequest, BuildError> {
        Ok(PromptsListQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            alias: self.alias,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            sort_field: self.sort_field,
            sort_direction_legacy: self.sort_direction_legacy,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
        })
    }
}
