pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ManagedIndexFieldsChunkFieldsValue {
    #[serde(rename = "bool")]
    #[non_exhaustive]
    Bool {
        #[serde(flatten)]
        data: BoolFieldDefinition,
    },

    #[serde(rename = "bool[]")]
    #[non_exhaustive]
    BoolArray {
        #[serde(flatten)]
        data: BoolArrayFieldDefinition,
    },

    #[serde(rename = "dense_vector")]
    #[non_exhaustive]
    DenseVector {
        #[serde(flatten)]
        data: DenseVectorFieldDefinition,
    },

    #[serde(rename = "float")]
    #[non_exhaustive]
    Float {
        #[serde(flatten)]
        data: FloatFieldDefinition,
    },

    #[serde(rename = "float[]")]
    #[non_exhaustive]
    FloatArray {
        #[serde(flatten)]
        data: FloatArrayFieldDefinition,
    },

    #[serde(rename = "int")]
    #[non_exhaustive]
    Int {
        #[serde(flatten)]
        data: IntFieldDefinition,
    },

    #[serde(rename = "int[]")]
    #[non_exhaustive]
    IntArray {
        #[serde(flatten)]
        data: IntArrayFieldDefinition,
    },

    #[serde(rename = "json")]
    #[non_exhaustive]
    Json {
        #[serde(flatten)]
        data: JsonFieldDefinition,
    },

    #[serde(rename = "keyword")]
    #[non_exhaustive]
    Keyword {
        #[serde(flatten)]
        data: KeywordFieldDefinition,
    },

    #[serde(rename = "keyword[]")]
    #[non_exhaustive]
    KeywordArray {
        #[serde(flatten)]
        data: KeywordArrayFieldDefinition,
    },

    #[serde(rename = "long")]
    #[non_exhaustive]
    Long {
        #[serde(flatten)]
        data: LongFieldDefinition,
    },

    #[serde(rename = "long[]")]
    #[non_exhaustive]
    LongArray {
        #[serde(flatten)]
        data: LongArrayFieldDefinition,
    },

    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(flatten)]
        data: TextFieldDefinition,
    },

    #[serde(rename = "text[]")]
    #[non_exhaustive]
    TextArray {
        #[serde(flatten)]
        data: TextArrayFieldDefinition,
    },

    #[serde(rename = "timestamp")]
    #[non_exhaustive]
    Timestamp {
        #[serde(flatten)]
        data: TimestampFieldDefinition,
    },

    #[serde(rename = "timestamp[]")]
    #[non_exhaustive]
    TimestampArray {
        #[serde(flatten)]
        data: TimestampArrayFieldDefinition,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ManagedIndexFieldsChunkFieldsValue {
    pub fn bool(data: BoolFieldDefinition) -> Self {
        Self::Bool { data }
    }

    pub fn bool_array(data: BoolArrayFieldDefinition) -> Self {
        Self::BoolArray { data }
    }

    pub fn dense_vector(data: DenseVectorFieldDefinition) -> Self {
        Self::DenseVector { data }
    }

    pub fn float(data: FloatFieldDefinition) -> Self {
        Self::Float { data }
    }

    pub fn float_array(data: FloatArrayFieldDefinition) -> Self {
        Self::FloatArray { data }
    }

    pub fn int(data: IntFieldDefinition) -> Self {
        Self::Int { data }
    }

    pub fn int_array(data: IntArrayFieldDefinition) -> Self {
        Self::IntArray { data }
    }

    pub fn json(data: JsonFieldDefinition) -> Self {
        Self::Json { data }
    }

    pub fn keyword(data: KeywordFieldDefinition) -> Self {
        Self::Keyword { data }
    }

    pub fn keyword_array(data: KeywordArrayFieldDefinition) -> Self {
        Self::KeywordArray { data }
    }

    pub fn long(data: LongFieldDefinition) -> Self {
        Self::Long { data }
    }

    pub fn long_array(data: LongArrayFieldDefinition) -> Self {
        Self::LongArray { data }
    }

    pub fn text(data: TextFieldDefinition) -> Self {
        Self::Text { data }
    }

    pub fn text_array(data: TextArrayFieldDefinition) -> Self {
        Self::TextArray { data }
    }

    pub fn timestamp(data: TimestampFieldDefinition) -> Self {
        Self::Timestamp { data }
    }

    pub fn timestamp_array(data: TimestampArrayFieldDefinition) -> Self {
        Self::TimestampArray { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
