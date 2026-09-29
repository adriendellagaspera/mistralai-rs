pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<AnnotationsAudienceItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
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
    priority: Option<f64>,
}

impl AnnotationsBuilder {
    pub fn audience(mut self, value: Vec<AnnotationsAudienceItem>) -> Self {
        self.audience = Some(value);
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
            priority: self.priority,
            extra: Default::default(),
        })
    }
}
