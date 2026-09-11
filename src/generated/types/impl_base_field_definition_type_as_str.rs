impl BaseFieldDefinitionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enum_ => "ENUM",
            Self::Text => "TEXT",
            Self::Int => "INT",
            Self::Float => "FLOAT",
            Self::Bool => "BOOL",
            Self::Timestamp => "TIMESTAMP",
            Self::Array => "ARRAY",
        }
    }
}
