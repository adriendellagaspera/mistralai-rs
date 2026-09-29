pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_type")]
#[non_exhaustive]
pub enum JobsApiRoutesFineTuningUpdateFineTunedModelModelsResponse {
    #[serde(rename = "classifier")]
    #[non_exhaustive]
    Classifier {
        #[serde(skip_serializing_if = "Option::is_none")]
        aliases: Option<Vec<String>>,
        #[serde(default)]
        archived: bool,
        #[serde(default)]
        capabilities: FineTunedModelCapabilities,
        #[serde(default)]
        classifier_targets: Vec<ClassifierTargetResult>,
        #[serde(default)]
        created: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        job: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_context_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        object: Option<ClassifierFineTunedModelObject>,
        #[serde(default)]
        owned_by: String,
        #[serde(default)]
        root: String,
        #[serde(default)]
        root_version: String,
        #[serde(default)]
        workspace_id: String,
    },

    #[serde(rename = "completion")]
    #[non_exhaustive]
    Completion {
        #[serde(skip_serializing_if = "Option::is_none")]
        aliases: Option<Vec<String>>,
        #[serde(default)]
        archived: bool,
        #[serde(default)]
        capabilities: FineTunedModelCapabilities,
        #[serde(default)]
        created: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        job: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_context_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        object: Option<CompletionFineTunedModelObject>,
        #[serde(default)]
        owned_by: String,
        #[serde(default)]
        root: String,
        #[serde(default)]
        root_version: String,
        #[serde(default)]
        workspace_id: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl JobsApiRoutesFineTuningUpdateFineTunedModelModelsResponse {
    pub fn classifier(
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        id: String,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases: None,
            archived,
            capabilities,
            classifier_targets,
            created,
            description: None,
            id,
            job: None,
            max_context_length: None,
            name: None,
            object: None,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion(
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        id: String,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases: None,
            archived,
            capabilities,
            created,
            description: None,
            id,
            job: None,
            max_context_length: None,
            name: None,
            object: None,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_aliases(
        aliases: Vec<String>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<ClassifierFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases: Some(aliases),
            archived,
            capabilities,
            classifier_targets,
            created,
            description,
            id,
            job,
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_description(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: String,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<ClassifierFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases,
            archived,
            capabilities,
            classifier_targets,
            created,
            description: Some(description),
            id,
            job,
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_job(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: Option<String>,
        id: String,
        job: String,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<ClassifierFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases,
            archived,
            capabilities,
            classifier_targets,
            created,
            description,
            id,
            job: Some(job),
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_max_context_length(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: i64,
        name: Option<String>,
        object: Option<ClassifierFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases,
            archived,
            capabilities,
            classifier_targets,
            created,
            description,
            id,
            job,
            max_context_length: Some(max_context_length),
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_name(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: String,
        object: Option<ClassifierFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases,
            archived,
            capabilities,
            classifier_targets,
            created,
            description,
            id,
            job,
            max_context_length,
            name: Some(name),
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn classifier_with_object(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: ClassifierFineTunedModelObject,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Classifier {
            aliases,
            archived,
            capabilities,
            classifier_targets,
            created,
            description,
            id,
            job,
            max_context_length,
            name,
            object: Some(object),
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_aliases(
        aliases: Vec<String>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<CompletionFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases: Some(aliases),
            archived,
            capabilities,
            created,
            description,
            id,
            job,
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_description(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: String,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<CompletionFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases,
            archived,
            capabilities,
            created,
            description: Some(description),
            id,
            job,
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_job(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: Option<String>,
        id: String,
        job: String,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: Option<CompletionFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases,
            archived,
            capabilities,
            created,
            description,
            id,
            job: Some(job),
            max_context_length,
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_max_context_length(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: i64,
        name: Option<String>,
        object: Option<CompletionFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases,
            archived,
            capabilities,
            created,
            description,
            id,
            job,
            max_context_length: Some(max_context_length),
            name,
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_name(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: String,
        object: Option<CompletionFineTunedModelObject>,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases,
            archived,
            capabilities,
            created,
            description,
            id,
            job,
            max_context_length,
            name: Some(name),
            object,
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn completion_with_object(
        aliases: Option<Vec<String>>,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        created: i64,
        description: Option<String>,
        id: String,
        job: Option<String>,
        max_context_length: Option<i64>,
        name: Option<String>,
        object: CompletionFineTunedModelObject,
        owned_by: String,
        root: String,
        root_version: String,
        workspace_id: String,
    ) -> Self {
        Self::Completion {
            aliases,
            archived,
            capabilities,
            created,
            description,
            id,
            job,
            max_context_length,
            name,
            object: Some(object),
            owned_by,
            root,
            root_version,
            workspace_id,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
