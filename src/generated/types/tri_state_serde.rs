/// Serde normally maps both a missing `Option<T>` field and an
/// explicit JSON null to `None`. Wrapping the decoded value in
/// `Some` retains the field-presence bit for `Option<Option<T>>`.
mod tri_state_serde {
    use serde::{Deserialize, Deserializer};
    pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        T::deserialize(de).map(Some)
    }
}
