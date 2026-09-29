pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Judge {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub owner_id: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub model_name: String,
    pub output: JudgeOutputConfig,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_revision: Option<String>,
}

impl Judge {
    pub fn builder() -> JudgeBuilder {
        <JudgeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    owner_id: Option<String>,
    workspace_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    model_name: Option<String>,
    output: Option<JudgeOutputConfig>,
    instructions: Option<String>,
    tools: Option<Vec<String>>,
    up_revision: Option<String>,
    down_revision: Option<String>,
    base_revision: Option<String>,
}

impl JudgeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = Some(value.into());
        self
    }

    pub fn output(mut self, value: JudgeOutputConfig) -> Self {
        self.output = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn up_revision(mut self, value: impl Into<String>) -> Self {
        self.up_revision = Some(value.into());
        self
    }

    pub fn down_revision(mut self, value: impl Into<String>) -> Self {
        self.down_revision = Some(value.into());
        self
    }

    pub fn base_revision(mut self, value: impl Into<String>) -> Self {
        self.base_revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Judge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JudgeBuilder::id)
    /// - [`created_at`](JudgeBuilder::created_at)
    /// - [`updated_at`](JudgeBuilder::updated_at)
    /// - [`owner_id`](JudgeBuilder::owner_id)
    /// - [`workspace_id`](JudgeBuilder::workspace_id)
    /// - [`name`](JudgeBuilder::name)
    /// - [`description`](JudgeBuilder::description)
    /// - [`model_name`](JudgeBuilder::model_name)
    /// - [`output`](JudgeBuilder::output)
    /// - [`instructions`](JudgeBuilder::instructions)
    /// - [`tools`](JudgeBuilder::tools)
    pub fn build(self) -> Result<Judge, BuildError> {
        Ok(Judge {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            deleted_at: self.deleted_at,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            model_name: self
                .model_name
                .ok_or_else(|| BuildError::missing_field("model_name"))?,
            output: self
                .output
                .ok_or_else(|| BuildError::missing_field("output"))?,
            instructions: self
                .instructions
                .ok_or_else(|| BuildError::missing_field("instructions"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
            up_revision: self.up_revision,
            down_revision: self.down_revision,
            base_revision: self.base_revision,
        })
    }
}
