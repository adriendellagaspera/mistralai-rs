#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRPageDimensions {
    ///Dots per inch of the page-image
    ///Constraint: minimum=0
    pub dpi: i64,
    ///Height of the image in pixels
    ///Constraint: minimum=0
    pub height: i64,
    ///Width of the image in pixels
    ///Constraint: minimum=0
    pub width: i64,
}
