#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobIn {
    ///This field will be required in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_start: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub classifier_targets: Option<Option<Vec<ClassifierTargetIn>>>,
    pub hyperparameters: JobInHyperparameters,
    ///A list of integrations to enable for your fine-tuning job.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub integrations: Option<Option<Vec<WandbIntegration>>>,
    ///Constraint: minimum=0, maximum=0.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_sample_skip_percentage: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub job_type: Option<Option<FineTuneableModelType>>,
    pub model: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub repositories: Option<Option<Vec<GithubRepositoryIn>>>,
    ///A string that will be added to your fine-tuning model name. For example, a suffix of "my-great-model" would produce a model name like `ft:open-mistral-7b:my-great-model:xxx...`
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub suffix: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_files: Option<Vec<TrainingFile>>,
    ///A list containing the IDs of uploaded files that contain validation data. If you provide these files, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in `checkpoints` when getting the status of a running fine-tuning job. The same data should not be present in both train and validation files.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub validation_files: Option<Option<Vec<String>>>,
}
