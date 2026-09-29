pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupsSummaryOut {
    #[serde(default)]
    pub create: Vec<ScimSyncGroupActionOut>,
    #[serde(default)]
    pub update: Vec<ScimSyncGroupUpdateActionOut>,
    #[serde(default)]
    pub delete: Vec<ScimSyncGroupActionOut>,
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
    update: Option<Vec<ScimSyncGroupUpdateActionOut>>,
    delete: Option<Vec<ScimSyncGroupActionOut>>,
}

impl ScimSyncGroupsSummaryOutBuilder {
    pub fn create(mut self, value: Vec<ScimSyncGroupActionOut>) -> Self {
        self.create = Some(value);
        self
    }

    pub fn update(mut self, value: Vec<ScimSyncGroupUpdateActionOut>) -> Self {
        self.update = Some(value);
        self
    }

    pub fn delete(mut self, value: Vec<ScimSyncGroupActionOut>) -> Self {
        self.delete = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupsSummaryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`create`](ScimSyncGroupsSummaryOutBuilder::create)
    /// - [`update`](ScimSyncGroupsSummaryOutBuilder::update)
    /// - [`delete`](ScimSyncGroupsSummaryOutBuilder::delete)
    pub fn build(self) -> Result<ScimSyncGroupsSummaryOut, BuildError> {
        Ok(ScimSyncGroupsSummaryOut {
            create: self
                .create
                .ok_or_else(|| BuildError::missing_field("create"))?,
            update: self
                .update
                .ok_or_else(|| BuildError::missing_field("update"))?,
            delete: self
                .delete
                .ok_or_else(|| BuildError::missing_field("delete"))?,
        })
    }
}
