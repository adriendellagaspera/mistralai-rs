pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileSchema {
    /// The size of the file, in bytes.
    #[serde(default)]
    pub bytes: i64,
    /// The UNIX timestamp (in seconds) of the event.
    #[serde(default)]
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// The name of the uploaded file.
    #[serde(default)]
    pub filename: String,
    /// The unique identifier of the file.
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_lines: Option<i64>,
    /// The object type, which is always "file".
    #[serde(default)]
    pub object: String,
    /// The intended purpose of the uploaded file, currently supports fine-tuning (`fine-tune`), OCR (`ocr`), Audio/Transcription (`audio`) and batch inference (`batch`).
    pub purpose: FilePurpose,
    pub sample_type: SampleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub source: Source,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<FileVisibility>,
}

impl FileSchema {
    pub fn builder() -> FileSchemaBuilder {
        <FileSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileSchemaBuilder {
    bytes: Option<i64>,
    created_at: Option<i64>,
    expires_at: Option<i64>,
    filename: Option<String>,
    id: Option<String>,
    mimetype: Option<String>,
    num_lines: Option<i64>,
    object: Option<String>,
    purpose: Option<FilePurpose>,
    sample_type: Option<SampleType>,
    signature: Option<String>,
    source: Option<Source>,
    visibility: Option<FileVisibility>,
}

impl FileSchemaBuilder {
    pub fn bytes(mut self, value: i64) -> Self {
        self.bytes = Some(value);
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mimetype(mut self, value: impl Into<String>) -> Self {
        self.mimetype = Some(value.into());
        self
    }

    pub fn num_lines(mut self, value: i64) -> Self {
        self.num_lines = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: FilePurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn sample_type(mut self, value: SampleType) -> Self {
        self.sample_type = Some(value);
        self
    }

    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.signature = Some(value.into());
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn visibility(mut self, value: FileVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bytes`](FileSchemaBuilder::bytes)
    /// - [`created_at`](FileSchemaBuilder::created_at)
    /// - [`filename`](FileSchemaBuilder::filename)
    /// - [`id`](FileSchemaBuilder::id)
    /// - [`object`](FileSchemaBuilder::object)
    /// - [`purpose`](FileSchemaBuilder::purpose)
    /// - [`sample_type`](FileSchemaBuilder::sample_type)
    /// - [`source`](FileSchemaBuilder::source)
    pub fn build(self) -> Result<FileSchema, BuildError> {
        Ok(FileSchema {
            bytes: self
                .bytes
                .ok_or_else(|| BuildError::missing_field("bytes"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self.expires_at,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mimetype: self.mimetype,
            num_lines: self.num_lines,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            sample_type: self
                .sample_type
                .ok_or_else(|| BuildError::missing_field("sample_type"))?,
            signature: self.signature,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            visibility: self.visibility,
        })
    }
}
