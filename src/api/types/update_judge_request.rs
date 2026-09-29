pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateJudgeRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub model_name: String,
    #[serde(default)]
    pub name: String,
    pub output: UpdateJudgeRequestOutput,
    #[serde(default)]
    pub tools: Vec<String>,
}

impl UpdateJudgeRequest {
    pub fn builder() -> UpdateJudgeRequestBuilder {
        <UpdateJudgeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateJudgeRequestBuilder {
    description: Option<String>,
    instructions: Option<String>,
    model_name: Option<String>,
    name: Option<String>,
    output: Option<UpdateJudgeRequestOutput>,
    tools: Option<Vec<String>>,
}

impl UpdateJudgeRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn output(mut self, value: UpdateJudgeRequestOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateJudgeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](UpdateJudgeRequestBuilder::description)
    /// - [`instructions`](UpdateJudgeRequestBuilder::instructions)
    /// - [`model_name`](UpdateJudgeRequestBuilder::model_name)
    /// - [`name`](UpdateJudgeRequestBuilder::name)
    /// - [`output`](UpdateJudgeRequestBuilder::output)
    /// - [`tools`](UpdateJudgeRequestBuilder::tools)
    pub fn build(self) -> Result<UpdateJudgeRequest, BuildError> {
        Ok(UpdateJudgeRequest {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            instructions: self
                .instructions
                .ok_or_else(|| BuildError::missing_field("instructions"))?,
            model_name: self
                .model_name
                .ok_or_else(|| BuildError::missing_field("model_name"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            output: self
                .output
                .ok_or_else(|| BuildError::missing_field("output"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
