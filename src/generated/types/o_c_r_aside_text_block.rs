#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRAsideTextBlock {
    ///Constraint: minimum=0
    pub bottom_right_x: i64,
    ///Constraint: minimum=0
    pub bottom_right_y: i64,
    ///Text/markdown/html content of this block
    pub content: String,
    ///Constraint: minimum=0
    pub top_left_x: i64,
    ///Constraint: minimum=0
    pub top_left_y: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OCRAsideTextBlockType>,
}
