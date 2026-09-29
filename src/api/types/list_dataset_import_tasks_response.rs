pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetImportTasksResponse {
    #[serde(default)]
    pub tasks: PaginatedResultDatasetImportTask,
}

impl ListDatasetImportTasksResponse {
    pub fn builder() -> ListDatasetImportTasksResponseBuilder {
        <ListDatasetImportTasksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetImportTasksResponseBuilder {
    tasks: Option<PaginatedResultDatasetImportTask>,
}

impl ListDatasetImportTasksResponseBuilder {
    pub fn tasks(mut self, value: PaginatedResultDatasetImportTask) -> Self {
        self.tasks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetImportTasksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tasks`](ListDatasetImportTasksResponseBuilder::tasks)
    pub fn build(self) -> Result<ListDatasetImportTasksResponse, BuildError> {
        Ok(ListDatasetImportTasksResponse {
            tasks: self
                .tasks
                .ok_or_else(|| BuildError::missing_field("tasks"))?,
        })
    }
}
