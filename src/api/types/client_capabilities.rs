pub use crate::prelude::*;

/// Capabilities a client may support.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation: Option<ElicitationCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<ClientTasksCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ClientCapabilities {
    pub fn builder() -> ClientCapabilitiesBuilder {
        <ClientCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCapabilitiesBuilder {
    elicitation: Option<ElicitationCapability>,
    experimental: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    roots: Option<RootsCapability>,
    sampling: Option<SamplingCapability>,
    tasks: Option<ClientTasksCapability>,
}

impl ClientCapabilitiesBuilder {
    pub fn elicitation(mut self, value: ElicitationCapability) -> Self {
        self.elicitation = Some(value);
        self
    }

    pub fn experimental(
        mut self,
        value: HashMap<String, Option<HashMap<String, serde_json::Value>>>,
    ) -> Self {
        self.experimental = Some(value);
        self
    }

    pub fn roots(mut self, value: RootsCapability) -> Self {
        self.roots = Some(value);
        self
    }

    pub fn sampling(mut self, value: SamplingCapability) -> Self {
        self.sampling = Some(value);
        self
    }

    pub fn tasks(mut self, value: ClientTasksCapability) -> Self {
        self.tasks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientCapabilities`].
    pub fn build(self) -> Result<ClientCapabilities, BuildError> {
        Ok(ClientCapabilities {
            elicitation: self.elicitation,
            experimental: self.experimental,
            roots: self.roots,
            sampling: self.sampling,
            tasks: self.tasks,
            extra: Default::default(),
        })
    }
}
