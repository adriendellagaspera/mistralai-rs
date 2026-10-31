pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Projects {
    #[serde(default)]
    pub projects: PaginatedResultProject,
}

impl Projects {
    pub fn builder() -> ProjectsBuilder {
        <ProjectsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsBuilder {
    projects: Option<PaginatedResultProject>,
}

impl ProjectsBuilder {
    pub fn projects(mut self, value: PaginatedResultProject) -> Self {
        self.projects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Projects`].
    /// This method will fail if any of the following fields are not set:
    /// - [`projects`](ProjectsBuilder::projects)
    pub fn build(self) -> Result<Projects, BuildError> {
        Ok(Projects {
            projects: self
                .projects
                .ok_or_else(|| BuildError::missing_field("projects"))?,
        })
    }
}
