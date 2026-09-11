#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRImageObject {
    ///X coordinate of bottom-right corner of the extracted image
    pub bottom_right_x: Option<i64>,
    ///Y coordinate of bottom-right corner of the extracted image
    pub bottom_right_y: Option<i64>,
    ///Image ID for extracted image in a page
    pub id: String,
    ///Annotation of the extracted image in json str
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub image_annotation: Option<Option<String>>,
    ///Base64 string of the extracted image
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub image_base64: Option<Option<String>>,
    ///X coordinate of top-left corner of the extracted image
    pub top_left_x: Option<i64>,
    ///Y coordinate of top-left corner of the extracted image
    pub top_left_y: Option<i64>,
}
