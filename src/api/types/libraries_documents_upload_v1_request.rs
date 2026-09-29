pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LibrariesDocumentsUploadV1Request {
    #[serde(default)]
    pub file: File,
}
impl LibrariesDocumentsUploadV1Request {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Ok(json_str) = serde_json::to_string(&self.file) {
            form = form.text("file", json_str);
        }

        form
    }
}

impl LibrariesDocumentsUploadV1Request {
    pub fn builder() -> LibrariesDocumentsUploadV1RequestBuilder {
        <LibrariesDocumentsUploadV1RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesDocumentsUploadV1RequestBuilder {
    file: Option<File>,
}

impl LibrariesDocumentsUploadV1RequestBuilder {
    pub fn file(mut self, value: File) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibrariesDocumentsUploadV1Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](LibrariesDocumentsUploadV1RequestBuilder::file)
    pub fn build(self) -> Result<LibrariesDocumentsUploadV1Request, BuildError> {
        Ok(LibrariesDocumentsUploadV1Request {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
