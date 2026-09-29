pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(default)]
    pub file: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<FilePurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<UploadFilesRequestVisibility>,
}
impl UploadRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref value) = self.expiry {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("expiry", json_str);
            }
        }

        if let Ok(json_str) = serde_json::to_string(&self.file) {
            form = form.text("file", json_str);
        }

        if let Some(ref value) = self.purpose {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("purpose", json_str);
            }
        }

        if let Some(ref value) = self.visibility {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("visibility", json_str);
            }
        }

        form
    }
}

impl UploadRequest {
    pub fn builder() -> UploadRequestBuilder {
        <UploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadRequestBuilder {
    expiry: Option<i64>,
    file: Option<File>,
    purpose: Option<FilePurpose>,
    visibility: Option<UploadFilesRequestVisibility>,
}

impl UploadRequestBuilder {
    pub fn expiry(mut self, value: i64) -> Self {
        self.expiry = Some(value);
        self
    }

    pub fn file(mut self, value: File) -> Self {
        self.file = Some(value);
        self
    }

    pub fn purpose(mut self, value: FilePurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn visibility(mut self, value: UploadFilesRequestVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadRequestBuilder::file)
    pub fn build(self) -> Result<UploadRequest, BuildError> {
        Ok(UploadRequest {
            expiry: self.expiry,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            purpose: self.purpose,
            visibility: self.visibility,
        })
    }
}
