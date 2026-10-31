pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScalarStatisticValueType {
    #[serde(rename = "scalar")]
    Scalar,
}
impl fmt::Display for ScalarStatisticValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Scalar => "scalar",
        };
        write!(f, "{}", s)
    }
}
