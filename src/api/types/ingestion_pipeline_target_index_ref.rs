pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngestionPipelineTargetIndexRef {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IngestionPipelineTargetIndexRefType>,
}

impl IngestionPipelineTargetIndexRef {
    pub fn builder() -> IngestionPipelineTargetIndexRefBuilder {
        <IngestionPipelineTargetIndexRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestionPipelineTargetIndexRefBuilder {
    name: Option<String>,
    r#type: Option<IngestionPipelineTargetIndexRefType>,
}

impl IngestionPipelineTargetIndexRefBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: IngestionPipelineTargetIndexRefType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestionPipelineTargetIndexRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](IngestionPipelineTargetIndexRefBuilder::name)
    pub fn build(self) -> Result<IngestionPipelineTargetIndexRef, BuildError> {
        Ok(IngestionPipelineTargetIndexRef {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
        })
    }
}
