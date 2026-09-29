pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSharingResponse {
    #[serde(default)]
    pub data: Vec<Sharing>,
}

impl ListSharingResponse {
    pub fn builder() -> ListSharingResponseBuilder {
        <ListSharingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSharingResponseBuilder {
    data: Option<Vec<Sharing>>,
}

impl ListSharingResponseBuilder {
    pub fn data(mut self, value: Vec<Sharing>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSharingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSharingResponseBuilder::data)
    pub fn build(self) -> Result<ListSharingResponse, BuildError> {
        Ok(ListSharingResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
