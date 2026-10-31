pub use crate::prelude::*;

/// Optional annotations the client can use to inform how objects are used or displayed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<AnnotationsAudienceItem>>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
}

impl Annotations {
    pub fn builder() -> AnnotationsBuilder {
        <AnnotationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnotationsBuilder {
    audience: Option<Vec<AnnotationsAudienceItem>>,
    last_modified: Option<String>,
    priority: Option<f64>,
}

impl AnnotationsBuilder {
    pub fn audience(mut self, value: Vec<AnnotationsAudienceItem>) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn last_modified(mut self, value: impl Into<String>) -> Self {
        self.last_modified = Some(value.into());
        self
    }

    pub fn priority(mut self, value: f64) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Annotations`].
    pub fn build(self) -> Result<Annotations, BuildError> {
        Ok(Annotations {
            audience: self.audience,
            last_modified: self.last_modified,
            priority: self.priority,
        })
    }
}
