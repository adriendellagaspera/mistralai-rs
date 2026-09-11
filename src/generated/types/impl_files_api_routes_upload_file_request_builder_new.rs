impl FilesApiRoutesUploadFileRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(file: File) -> Self {
        Self {
            value: FilesApiRoutesUploadFileRequest::new(file),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "expiry", "` request field to a value."
    )]
    #[must_use]
    pub fn expiry(mut self, expiry: i64) -> Self {
        self.value.expiry = Some(Some(expiry));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "expiry", "` request field to JSON null."
    )]
    #[must_use]
    pub fn expiry_null(mut self) -> Self {
        self.value.expiry = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "expiry", "` request field.")]
    #[must_use]
    pub fn expiry_absent(mut self) -> Self {
        self.value.expiry = None;
        self
    }
    #[doc = concat!("Set the optional `", "purpose", "` request field.")]
    #[must_use]
    pub fn purpose(mut self, purpose: FilePurpose) -> Self {
        self.value.purpose = Some(purpose);
        self
    }
    #[doc = concat!("Set the optional `", "visibility", "` request field.")]
    #[must_use]
    pub fn visibility(
        mut self,
        visibility: FilesApiRoutesUploadFileRequestVisibilityWorkspaceEnum,
    ) -> Self {
        self.value.visibility = Some(visibility);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> FilesApiRoutesUploadFileRequest {
        self.value
    }
}
