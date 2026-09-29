pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsWorkerSpecUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl WorkflowsWorkerSpecUpdate {
    pub fn builder() -> WorkflowsWorkerSpecUpdateBuilder {
        <WorkflowsWorkerSpecUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsWorkerSpecUpdateBuilder {
    entrypoint: Option<String>,
    github_url: Option<String>,
    revision: Option<String>,
    working_dir: Option<String>,
}

impl WorkflowsWorkerSpecUpdateBuilder {
    pub fn entrypoint(mut self, value: impl Into<String>) -> Self {
        self.entrypoint = Some(value.into());
        self
    }

    pub fn github_url(mut self, value: impl Into<String>) -> Self {
        self.github_url = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn working_dir(mut self, value: impl Into<String>) -> Self {
        self.working_dir = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsWorkerSpecUpdate`].
    pub fn build(self) -> Result<WorkflowsWorkerSpecUpdate, BuildError> {
        Ok(WorkflowsWorkerSpecUpdate {
            entrypoint: self.entrypoint,
            github_url: self.github_url,
            revision: self.revision,
            working_dir: self.working_dir,
        })
    }
}
