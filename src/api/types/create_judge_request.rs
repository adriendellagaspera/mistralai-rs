pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateJudgeRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub model_name: String,
    pub output: CreateJudgeRequestOutput,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tools: Vec<String>,
}

impl CreateJudgeRequest {
    pub fn builder() -> CreateJudgeRequestBuilder {
        <CreateJudgeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateJudgeRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    model_name: Option<String>,
    output: Option<CreateJudgeRequestOutput>,
    instructions: Option<String>,
    tools: Option<Vec<String>>,
}

impl CreateJudgeRequestBuilder {
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

    pub fn output(mut self, value: CreateJudgeRequestOutput) -> Self {
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

    /// Consumes the builder and constructs a [`CreateJudgeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateJudgeRequestBuilder::name)
    /// - [`description`](CreateJudgeRequestBuilder::description)
    /// - [`model_name`](CreateJudgeRequestBuilder::model_name)
    /// - [`output`](CreateJudgeRequestBuilder::output)
    /// - [`instructions`](CreateJudgeRequestBuilder::instructions)
    /// - [`tools`](CreateJudgeRequestBuilder::tools)
    pub fn build(self) -> Result<CreateJudgeRequest, BuildError> {
        Ok(CreateJudgeRequest {
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
