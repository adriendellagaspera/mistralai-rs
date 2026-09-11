/// UTF-8 JSON string codec for `bytes::Bytes` model fields
/// produced from `format: binary`. Raw HTTP body and multipart
/// paths use their byte carriers directly and do not invoke it.
mod binary_bytes_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(bytes: &bytes::Bytes, ser: S) -> Result<S::Ok, S::Error> {
        let value = std::str::from_utf8(bytes.as_ref()).map_err(serde::ser::Error::custom)?;
        ser.serialize_str(value)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<bytes::Bytes, D::Error> {
        String::deserialize(de).map(bytes::Bytes::from)
    }
    pub mod option {
        use serde::{Deserialize, Deserializer, Serializer};
        pub fn serialize<S: Serializer>(
            value: &Option<bytes::Bytes>,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(bytes) => super::serialize(bytes, ser),
                None => ser.serialize_none(),
            }
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(
            de: D,
        ) -> Result<Option<bytes::Bytes>, D::Error> {
            Option::<String>::deserialize(de).map(|value| value.map(bytes::Bytes::from))
        }
    }
    pub mod double_option {
        use serde::{Deserializer, Serializer};
        pub fn serialize<S: Serializer>(
            value: &Option<Option<bytes::Bytes>>,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(value) => super::option::serialize(value, ser),
                None => ser.serialize_none(),
            }
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(
            de: D,
        ) -> Result<Option<Option<bytes::Bytes>>, D::Error> {
            super::option::deserialize(de).map(Some)
        }
    }
}
