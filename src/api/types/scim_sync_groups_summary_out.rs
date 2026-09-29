pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupsSummaryOut {
    #[serde(default)]
    pub create: Vec<ScimSyncGroupActionOut>,
    #[serde(default)]
    pub delete: Vec<ScimSyncGroupActionOut>,
    #[serde(default)]
    pub update: Vec<ScimSyncGroupUpdateActionOut>,
}

impl ScimSyncGroupsSummaryOut {
    pub fn builder() -> ScimSyncGroupsSummaryOutBuilder {
        <ScimSyncGroupsSummaryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncGroupsSummaryOutBuilder {
    create: Option<Vec<ScimSyncGroupActionOut>>,
    delete: Option<Vec<ScimSyncGroupActionOut>>,
    update: Option<Vec<ScimSyncGroupUpdateActionOut>>,
}

impl ScimSyncGroupsSummaryOutBuilder {
    pub fn create(mut self, value: Vec<ScimSyncGroupActionOut>) -> Self {
        self.create = Some(value);
        self
    }

    pub fn delete(mut self, value: Vec<ScimSyncGroupActionOut>) -> Self {
        self.delete = Some(value);
        self
    }

    pub fn update(mut self, value: Vec<ScimSyncGroupUpdateActionOut>) -> Self {
        self.update = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupsSummaryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`create`](ScimSyncGroupsSummaryOutBuilder::create)
    /// - [`delete`](ScimSyncGroupsSummaryOutBuilder::delete)
    /// - [`update`](ScimSyncGroupsSummaryOutBuilder::update)
    pub fn build(self) -> Result<ScimSyncGroupsSummaryOut, BuildError> {
        Ok(ScimSyncGroupsSummaryOut {
            create: self
                .create
                .ok_or_else(|| BuildError::missing_field("create"))?,
            delete: self
                .delete
                .ok_or_else(|| BuildError::missing_field("delete"))?,
            update: self
                .update
                .ok_or_else(|| BuildError::missing_field("update"))?,
        })
    }
}
