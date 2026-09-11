impl FilesApiRoutesUploadFileRequest {
    /// Construct this request with every required wire field.
    pub fn new(file: File) -> Self {
        Self {
            file,
            expiry: None,
            purpose: None,
            visibility: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(file: File) -> FilesApiRoutesUploadFileRequestBuilder {
        FilesApiRoutesUploadFileRequestBuilder::new(file)
    }
}
