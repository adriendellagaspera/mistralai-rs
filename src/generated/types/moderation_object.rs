#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModerationObject {
    ///Moderation result thresholds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<ModerationObjectCategories>,
    ///Moderation result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_scores: Option<ModerationObjectCategoryScores>,
}
