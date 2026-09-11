impl CodeInterpreterToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CodeInterpreter => "code_interpreter",
        }
    }
}
