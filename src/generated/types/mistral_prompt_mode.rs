/**Available options to the prompt_mode argument on the chat completion endpoint.
Values represent high-level intent. Assignment to actual SPs is handled internally.
System prompt may include knowledge cutoff date, model capabilities, tone to use, safety guidelines, etc.*/
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MistralPromptMode {
    #[default]
    #[serde(rename = "reasoning")]
    Reasoning,
}
