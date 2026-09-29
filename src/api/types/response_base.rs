pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponseBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageInfo>,
}

impl ResponseBase {
    pub fn builder() -> ResponseBaseBuilder {
        <ResponseBaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseBaseBuilder {
    id: Option<String>,
    object: Option<String>,
    model: Option<String>,
    usage: Option<UsageInfo>,
}

impl ResponseBaseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn usage(mut self, value: UsageInfo) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponseBase`].
    pub fn build(self) -> Result<ResponseBase, BuildError> {
        Ok(ResponseBase {
            id: self.id,
            object: self.object,
            model: self.model,
            usage: self.usage,
        })
    }
}
