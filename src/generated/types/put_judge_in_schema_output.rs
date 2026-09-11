#[derive(Debug, Clone)]
pub enum PutJudgeInSchemaOutput {
    JudgeClassificationOutput(JudgeClassificationOutput),
    JudgeRegressionOutput(JudgeRegressionOutput),
}
