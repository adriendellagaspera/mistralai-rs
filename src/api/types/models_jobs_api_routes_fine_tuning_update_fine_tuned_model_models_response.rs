pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_type")]
#[non_exhaustive]
pub enum JobsApiRoutesFineTuningUpdateFineTunedModelModelsResponse {
    #[serde(rename = "classifier")]
    #[non_exhaustive]
    Classifier {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        object: Option<ClassifierFineTunedModelObject>,
        #[serde(default)]
        created: i64,
        #[serde(default)]
        owned_by: String,
        #[serde(default)]
        workspace_id: String,
        #[serde(default)]
        root: String,
        #[serde(default)]
        root_version: String,
        #[serde(default)]
        archived: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        capabilities: FineTunedModelCapabilities,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_context_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        aliases: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        job: Option<String>,
        #[serde(default)]
        classifier_targets: Vec<ClassifierTargetResult>,
    },

    #[serde(rename = "completion")]
    #[non_exhaustive]
    Completion {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        object: Option<CompletionFineTunedModelObject>,
        #[serde(default)]
        created: i64,
        #[serde(default)]
        owned_by: String,
        #[serde(default)]
        workspace_id: String,
        #[serde(default)]
        root: String,
        #[serde(default)]
        root_version: String,
        #[serde(default)]
        archived: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        capabilities: FineTunedModelCapabilities,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_context_length: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        aliases: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        job: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl JobsApiRoutesFineTuningUpdateFineTunedModelModelsResponse {
    pub fn classifier(
        id: String,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object: None,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name: None,
            description: None,
            capabilities,
            max_context_length: None,
            aliases: None,
            job: None,
            classifier_targets,
        }
    }

    pub fn completion(
        id: String,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        capabilities: FineTunedModelCapabilities,
    ) -> Self {
        Self::Completion {
            id,
            object: None,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name: None,
            description: None,
            capabilities,
            max_context_length: None,
            aliases: None,
            job: None,
        }
    }

    pub fn classifier_with_object(
        id: String,
        object: ClassifierFineTunedModelObject,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object: Some(object),
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases,
            job,
            classifier_targets,
        }
    }

    pub fn classifier_with_name(
        id: String,
        object: Option<ClassifierFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: String,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name: Some(name),
            description,
            capabilities,
            max_context_length,
            aliases,
            job,
            classifier_targets,
        }
    }

    pub fn classifier_with_description(
        id: String,
        object: Option<ClassifierFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: String,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description: Some(description),
            capabilities,
            max_context_length,
            aliases,
            job,
            classifier_targets,
        }
    }

    pub fn classifier_with_max_context_length(
        id: String,
        object: Option<ClassifierFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: i64,
        aliases: Option<Vec<String>>,
        job: Option<String>,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length: Some(max_context_length),
            aliases,
            job,
            classifier_targets,
        }
    }

    pub fn classifier_with_aliases(
        id: String,
        object: Option<ClassifierFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Vec<String>,
        job: Option<String>,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases: Some(aliases),
            job,
            classifier_targets,
        }
    }

    pub fn classifier_with_job(
        id: String,
        object: Option<ClassifierFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: String,
        classifier_targets: Vec<ClassifierTargetResult>,
    ) -> Self {
        Self::Classifier {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases,
            job: Some(job),
            classifier_targets,
        }
    }

    pub fn completion_with_object(
        id: String,
        object: CompletionFineTunedModelObject,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
    ) -> Self {
        Self::Completion {
            id,
            object: Some(object),
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases,
            job,
        }
    }

    pub fn completion_with_name(
        id: String,
        object: Option<CompletionFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: String,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
    ) -> Self {
        Self::Completion {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name: Some(name),
            description,
            capabilities,
            max_context_length,
            aliases,
            job,
        }
    }

    pub fn completion_with_description(
        id: String,
        object: Option<CompletionFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: String,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: Option<String>,
    ) -> Self {
        Self::Completion {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description: Some(description),
            capabilities,
            max_context_length,
            aliases,
            job,
        }
    }

    pub fn completion_with_max_context_length(
        id: String,
        object: Option<CompletionFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: i64,
        aliases: Option<Vec<String>>,
        job: Option<String>,
    ) -> Self {
        Self::Completion {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length: Some(max_context_length),
            aliases,
            job,
        }
    }

    pub fn completion_with_aliases(
        id: String,
        object: Option<CompletionFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Vec<String>,
        job: Option<String>,
    ) -> Self {
        Self::Completion {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases: Some(aliases),
            job,
        }
    }

    pub fn completion_with_job(
        id: String,
        object: Option<CompletionFineTunedModelObject>,
        created: i64,
        owned_by: String,
        workspace_id: String,
        root: String,
        root_version: String,
        archived: bool,
        name: Option<String>,
        description: Option<String>,
        capabilities: FineTunedModelCapabilities,
        max_context_length: Option<i64>,
        aliases: Option<Vec<String>>,
        job: String,
    ) -> Self {
        Self::Completion {
            id,
            object,
            created,
            owned_by,
            workspace_id,
            root,
            root_version,
            archived,
            name,
            description,
            capabilities,
            max_context_length,
            aliases,
            job: Some(job),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
