#[derive(Debug, Clone)]
pub enum OCRPageObjectBlocksItemUnion {
    OCRTextBlock(OCRTextBlock),
    OCRListBlock(OCRListBlock),
    OCRImageBlock(OCRImageBlock),
    OCRTableBlock(OCRTableBlock),
    OCRTitleBlock(OCRTitleBlock),
    OCREquationBlock(OCREquationBlock),
    OCRCaptionBlock(OCRCaptionBlock),
    OCRCodeBlock(OCRCodeBlock),
    OCRReferencesBlock(OCRReferencesBlock),
    OCRAsideTextBlock(OCRAsideTextBlock),
    OCRHeaderBlock(OCRHeaderBlock),
    OCRFooterBlock(OCRFooterBlock),
    OCRSignatureBlock(OCRSignatureBlock),
}
