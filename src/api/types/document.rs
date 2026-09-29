pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub library_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<i64>,
    /// Processing status of the document.
    pub process_status: ProcessStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_by_id: Option<String>,
    #[serde(default)]
    pub uploaded_by_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_processing_main_content: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_processing_summary: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// If set, the document will be automatically deleted after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_processing_total: Option<i64>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        <DocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentBuilder {
    id: Option<String>,
    library_id: Option<String>,
    hash: Option<String>,
    mime_type: Option<String>,
    extension: Option<String>,
    size: Option<i64>,
    name: Option<String>,
    summary: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    last_processed_at: Option<DateTime<FixedOffset>>,
    number_of_pages: Option<i64>,
    process_status: Option<ProcessStatus>,
    uploaded_by_id: Option<String>,
    uploaded_by_type: Option<String>,
    tokens_processing_main_content: Option<i64>,
    tokens_processing_summary: Option<i64>,
    url: Option<String>,
    attributes: Option<HashMap<String, serde_json::Value>>,
    expires_at: Option<DateTime<FixedOffset>>,
    processing_status: Option<String>,
    tokens_processing_total: Option<i64>,
}

impl DocumentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }

    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn extension(mut self, value: impl Into<String>) -> Self {
        self.extension = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last_processed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_processed_at = Some(value);
        self
    }

    pub fn number_of_pages(mut self, value: i64) -> Self {
        self.number_of_pages = Some(value);
        self
    }

    pub fn process_status(mut self, value: ProcessStatus) -> Self {
        self.process_status = Some(value);
        self
    }

    pub fn uploaded_by_id(mut self, value: impl Into<String>) -> Self {
        self.uploaded_by_id = Some(value.into());
        self
    }

    pub fn uploaded_by_type(mut self, value: impl Into<String>) -> Self {
        self.uploaded_by_type = Some(value.into());
        self
    }

    pub fn tokens_processing_main_content(mut self, value: i64) -> Self {
        self.tokens_processing_main_content = Some(value);
        self
    }

    pub fn tokens_processing_summary(mut self, value: i64) -> Self {
        self.tokens_processing_summary = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn processing_status(mut self, value: impl Into<String>) -> Self {
        self.processing_status = Some(value.into());
        self
    }

    pub fn tokens_processing_total(mut self, value: i64) -> Self {
        self.tokens_processing_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Document`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentBuilder::id)
    /// - [`library_id`](DocumentBuilder::library_id)
    /// - [`name`](DocumentBuilder::name)
    /// - [`created_at`](DocumentBuilder::created_at)
    /// - [`process_status`](DocumentBuilder::process_status)
    /// - [`uploaded_by_type`](DocumentBuilder::uploaded_by_type)
    pub fn build(self) -> Result<Document, BuildError> {
        Ok(Document {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            library_id: self
                .library_id
                .ok_or_else(|| BuildError::missing_field("library_id"))?,
            hash: self.hash,
            mime_type: self.mime_type,
            extension: self.extension,
            size: self.size,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            summary: self.summary,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_processed_at: self.last_processed_at,
            number_of_pages: self.number_of_pages,
            process_status: self
                .process_status
                .ok_or_else(|| BuildError::missing_field("process_status"))?,
            uploaded_by_id: self.uploaded_by_id,
            uploaded_by_type: self
                .uploaded_by_type
                .ok_or_else(|| BuildError::missing_field("uploaded_by_type"))?,
            tokens_processing_main_content: self.tokens_processing_main_content,
            tokens_processing_summary: self.tokens_processing_summary,
            url: self.url,
            attributes: self.attributes,
            expires_at: self.expires_at,
            processing_status: self.processing_status,
            tokens_processing_total: self.tokens_processing_total,
        })
    }
}
