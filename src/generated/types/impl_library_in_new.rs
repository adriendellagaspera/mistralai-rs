impl LibraryIn {
    /// Construct this request with every required wire field.
    pub fn new(name: String) -> Self {
        Self {
            name,
            chunk_size: None,
            description: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(name: String) -> LibraryInBuilder {
        LibraryInBuilder::new(name)
    }
}
