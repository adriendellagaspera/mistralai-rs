pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateJudgeRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub model_name: String,
    pub output: UpdateJudgeRequestOutput,
    #[serde(default)]
    pub instructions: String,
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
    name: Option<String>,
    description: Option<String>,
    model_name: Option<String>,
    output: Option<UpdateJudgeRequestOutput>,
    instructions: Option<String>,
    tools: Option<Vec<String>>,
}

impl UpdateJudgeRequestBuilder {
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

    pub fn output(mut self, value: UpdateJudgeRequestOutput) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateJudgeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateJudgeRequestBuilder::name)
    /// - [`description`](UpdateJudgeRequestBuilder::description)
    /// - [`model_name`](UpdateJudgeRequestBuilder::model_name)
    /// - [`output`](UpdateJudgeRequestBuilder::output)
    /// - [`instructions`](UpdateJudgeRequestBuilder::instructions)
    /// - [`tools`](UpdateJudgeRequestBuilder::tools)
    pub fn build(self) -> Result<UpdateJudgeRequest, BuildError> {
        Ok(UpdateJudgeRequest {
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
        })
    }
}
