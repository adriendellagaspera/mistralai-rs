impl OCREquationBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equation => "equation",
        }
    }
}
