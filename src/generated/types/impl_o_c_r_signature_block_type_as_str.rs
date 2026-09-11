impl OCRSignatureBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Signature => "signature",
        }
    }
}
