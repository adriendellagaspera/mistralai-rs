pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateJudgeRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub model_name: String,
    #[serde(default)]
    pub name: String,
    pub output: CreateJudgeRequestOutput,
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
    description: Option<String>,
    instructions: Option<String>,
    model_name: Option<String>,
    name: Option<String>,
    output: Option<CreateJudgeRequestOutput>,
    tools: Option<Vec<String>>,
}

impl CreateJudgeRequestBuilder {
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

    pub fn output(mut self, value: CreateJudgeRequestOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateJudgeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateJudgeRequestBuilder::description)
    /// - [`instructions`](CreateJudgeRequestBuilder::instructions)
    /// - [`model_name`](CreateJudgeRequestBuilder::model_name)
    /// - [`name`](CreateJudgeRequestBuilder::name)
    /// - [`output`](CreateJudgeRequestBuilder::output)
    /// - [`tools`](CreateJudgeRequestBuilder::tools)
    pub fn build(self) -> Result<CreateJudgeRequest, BuildError> {
        Ok(CreateJudgeRequest {
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
