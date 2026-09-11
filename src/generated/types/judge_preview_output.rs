#[derive(Debug, Clone)]
pub enum JudgePreviewOutput {
    JudgeClassificationOutput(JudgeClassificationOutput),
    JudgeRegressionOutput(JudgeRegressionOutput),
}
