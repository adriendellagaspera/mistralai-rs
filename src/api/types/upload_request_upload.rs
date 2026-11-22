pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadRequest2 {
    #[serde(default)]
    pub file: File,
}
impl UploadRequest2 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Ok(json_str) = serde_json::to_string(&self.file) {
            form = form.text("file", json_str);
        }

        form
    }
}

impl UploadRequest2 {
    pub fn builder() -> UploadRequest2Builder {
        <UploadRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadRequest2Builder {
    file: Option<File>,
}

impl UploadRequest2Builder {
    pub fn file(mut self, value: File) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadRequest2Builder::file)
    pub fn build(self) -> Result<UploadRequest2, BuildError> {
        Ok(UploadRequest2 {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
