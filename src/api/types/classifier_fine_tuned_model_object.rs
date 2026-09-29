pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClassifierFineTunedModelObject {
    #[serde(rename = "model")]
    Model,
}
impl fmt::Display for ClassifierFineTunedModelObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Model => "model",
        };
        write!(f, "{}", s)
    }
}
