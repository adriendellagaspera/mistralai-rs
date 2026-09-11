impl JobInBuilder {
    /// Start a builder with every required wire field.
    pub fn new(hyperparameters: JobInHyperparameters, model: String) -> Self {
        Self {
            value: JobIn::new(hyperparameters, model),
        }
    }
    #[doc = concat!("Set the optional `", "auto_start", "` request field.")]
    #[must_use]
    pub fn auto_start(mut self, auto_start: bool) -> Self {
        self.value.auto_start = Some(auto_start);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "classifier_targets",
        "` request field to a value."
    )]
    #[must_use]
    pub fn classifier_targets(mut self, classifier_targets: Vec<ClassifierTargetIn>) -> Self {
        self.value.classifier_targets = Some(Some(classifier_targets));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "classifier_targets",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn classifier_targets_null(mut self) -> Self {
        self.value.classifier_targets = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "classifier_targets", "` request field."
    )]
    #[must_use]
    pub fn classifier_targets_absent(mut self) -> Self {
        self.value.classifier_targets = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "integrations", "` request field to a value."
    )]
    #[must_use]
    pub fn integrations(mut self, integrations: Vec<WandbIntegration>) -> Self {
        self.value.integrations = Some(Some(integrations));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "integrations", "` request field to JSON null."
    )]
    #[must_use]
    pub fn integrations_null(mut self) -> Self {
        self.value.integrations = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "integrations", "` request field.")]
    #[must_use]
    pub fn integrations_absent(mut self) -> Self {
        self.value.integrations = None;
        self
    }
    #[doc = concat!(
        "Set the optional `", "invalid_sample_skip_percentage", "` request field."
    )]
    #[must_use]
    pub fn invalid_sample_skip_percentage(mut self, invalid_sample_skip_percentage: f64) -> Self {
        self.value.invalid_sample_skip_percentage = Some(invalid_sample_skip_percentage);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "job_type", "` request field to a value."
    )]
    #[must_use]
    pub fn job_type(mut self, job_type: FineTuneableModelType) -> Self {
        self.value.job_type = Some(Some(job_type));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "job_type", "` request field to JSON null."
    )]
    #[must_use]
    pub fn job_type_null(mut self) -> Self {
        self.value.job_type = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "job_type", "` request field.")]
    #[must_use]
    pub fn job_type_absent(mut self) -> Self {
        self.value.job_type = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "repositories", "` request field to a value."
    )]
    #[must_use]
    pub fn repositories(mut self, repositories: Vec<GithubRepositoryIn>) -> Self {
        self.value.repositories = Some(Some(repositories));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "repositories", "` request field to JSON null."
    )]
    #[must_use]
    pub fn repositories_null(mut self) -> Self {
        self.value.repositories = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "repositories", "` request field.")]
    #[must_use]
    pub fn repositories_absent(mut self) -> Self {
        self.value.repositories = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "suffix", "` request field to a value."
    )]
    #[must_use]
    pub fn suffix(mut self, suffix: String) -> Self {
        self.value.suffix = Some(Some(suffix));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "suffix", "` request field to JSON null."
    )]
    #[must_use]
    pub fn suffix_null(mut self) -> Self {
        self.value.suffix = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "suffix", "` request field.")]
    #[must_use]
    pub fn suffix_absent(mut self) -> Self {
        self.value.suffix = None;
        self
    }
    #[doc = concat!("Set the optional `", "training_files", "` request field.")]
    #[must_use]
    pub fn training_files(mut self, training_files: Vec<TrainingFile>) -> Self {
        self.value.training_files = Some(training_files);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "validation_files", "` request field to a value."
    )]
    #[must_use]
    pub fn validation_files(mut self, validation_files: Vec<String>) -> Self {
        self.value.validation_files = Some(Some(validation_files));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "validation_files",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn validation_files_null(mut self) -> Self {
        self.value.validation_files = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "validation_files", "` request field."
    )]
    #[must_use]
    pub fn validation_files_absent(mut self) -> Self {
        self.value.validation_files = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> JobIn {
        self.value
    }
}
