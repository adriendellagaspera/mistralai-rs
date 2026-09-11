impl JSONPatchAppendOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Append => "append",
        }
    }
}
