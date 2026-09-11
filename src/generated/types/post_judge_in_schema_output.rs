#[derive(Debug, Clone)]
pub enum PostJudgeInSchemaOutput {
    JudgeClassificationOutput(JudgeClassificationOutput),
    JudgeRegressionOutput(JudgeRegressionOutput),
}
