#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostChatCompletionEventJudgingInSchema {
    pub judge_definition: PostJudgeInSchema,
}
