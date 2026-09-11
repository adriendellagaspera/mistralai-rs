impl JobIn {
    /// Construct this request with every required wire field.
    pub fn new(hyperparameters: JobInHyperparameters, model: String) -> Self {
        Self {
            hyperparameters,
            model,
            auto_start: None,
            classifier_targets: None,
            integrations: None,
            invalid_sample_skip_percentage: None,
            job_type: None,
            repositories: None,
            suffix: None,
            training_files: None,
            validation_files: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(hyperparameters: JobInHyperparameters, model: String) -> JobInBuilder {
        JobInBuilder::new(hyperparameters, model)
    }
}
