pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticWorkspaceRoles {
    D7Ea77C5926041D0Ab2652B5Add3Ee56,
    UndefinedEe5644Bd8A2D712233977821,
    ThreeHundredSeventyFiveCd0Db3Bbe4B7980F3954Ccf04F3D1,
    UndefinedF143194C88994838A5184483B6,
    D79B30274Eb245218722825Acfee7D8B,
    TwoHundredFiftyTwoA082540B94B98Be807658956F13E9,
    SeventeenAa61C51C61477Ea40Ae52C8Ccd74B9,
    B23Cd6E091Cd4A8A9869B30366Bf3966,
    SevenHundredThirtyOneEb2BeA74F4070B79735Bf7009E553,
    Ff86D4327F2747F8B02Fb5C102Ef6A55,
    ZeroF9Acbf693B542C7A227Fc7652755F65,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StaticWorkspaceRoles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::D7Ea77C5926041D0Ab2652B5Add3Ee56 => {
                serializer.serialize_str("d7ea77c5-9260-41d0-ab26-52b5add3ee56")
            }
            Self::UndefinedEe5644Bd8A2D712233977821 => {
                serializer.serialize_str("48436751-ee56-44bd-8a2d-712233977821")
            }
            Self::ThreeHundredSeventyFiveCd0Db3Bbe4B7980F3954Ccf04F3D1 => {
                serializer.serialize_str("375cd0db-3bbe-4b79-80f3-954ccf04f3d1")
            }
            Self::UndefinedF143194C88994838A5184483B6 => {
                serializer.serialize_str("578584f1-4319-4c88-9948-38a5184483b6")
            }
            Self::D79B30274Eb245218722825Acfee7D8B => {
                serializer.serialize_str("d79b3027-4eb2-4521-8722-825acfee7d8b")
            }
            Self::TwoHundredFiftyTwoA082540B94B98Be807658956F13E9 => {
                serializer.serialize_str("252a0825-40b9-4b98-be80-7658956f13e9")
            }
            Self::SeventeenAa61C51C61477Ea40Ae52C8Ccd74B9 => {
                serializer.serialize_str("17aa61c5-1c61-477e-a40a-e52c8ccd74b9")
            }
            Self::B23Cd6E091Cd4A8A9869B30366Bf3966 => {
                serializer.serialize_str("b23cd6e0-91cd-4a8a-9869-b30366bf3966")
            }
            Self::SevenHundredThirtyOneEb2BeA74F4070B79735Bf7009E553 => {
                serializer.serialize_str("731eb2be-a74f-4070-b797-35bf7009e553")
            }
            Self::Ff86D4327F2747F8B02Fb5C102Ef6A55 => {
                serializer.serialize_str("ff86d432-7f27-47f8-b02f-b5c102ef6a55")
            }
            Self::ZeroF9Acbf693B542C7A227Fc7652755F65 => {
                serializer.serialize_str("0f9acbf6-93b5-42c7-a227-fc7652755f65")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StaticWorkspaceRoles {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "d7ea77c5-9260-41d0-ab26-52b5add3ee56" => Ok(Self::D7Ea77C5926041D0Ab2652B5Add3Ee56),
            "48436751-ee56-44bd-8a2d-712233977821" => Ok(Self::UndefinedEe5644Bd8A2D712233977821),
            "375cd0db-3bbe-4b79-80f3-954ccf04f3d1" => {
                Ok(Self::ThreeHundredSeventyFiveCd0Db3Bbe4B7980F3954Ccf04F3D1)
            }
            "578584f1-4319-4c88-9948-38a5184483b6" => Ok(Self::UndefinedF143194C88994838A5184483B6),
            "d79b3027-4eb2-4521-8722-825acfee7d8b" => Ok(Self::D79B30274Eb245218722825Acfee7D8B),
            "252a0825-40b9-4b98-be80-7658956f13e9" => {
                Ok(Self::TwoHundredFiftyTwoA082540B94B98Be807658956F13E9)
            }
            "17aa61c5-1c61-477e-a40a-e52c8ccd74b9" => {
                Ok(Self::SeventeenAa61C51C61477Ea40Ae52C8Ccd74B9)
            }
            "b23cd6e0-91cd-4a8a-9869-b30366bf3966" => Ok(Self::B23Cd6E091Cd4A8A9869B30366Bf3966),
            "731eb2be-a74f-4070-b797-35bf7009e553" => {
                Ok(Self::SevenHundredThirtyOneEb2BeA74F4070B79735Bf7009E553)
            }
            "ff86d432-7f27-47f8-b02f-b5c102ef6a55" => Ok(Self::Ff86D4327F2747F8B02Fb5C102Ef6A55),
            "0f9acbf6-93b5-42c7-a227-fc7652755f65" => Ok(Self::ZeroF9Acbf693B542C7A227Fc7652755F65),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StaticWorkspaceRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::D7Ea77C5926041D0Ab2652B5Add3Ee56 => {
                write!(f, "d7ea77c5-9260-41d0-ab26-52b5add3ee56")
            }
            Self::UndefinedEe5644Bd8A2D712233977821 => {
                write!(f, "48436751-ee56-44bd-8a2d-712233977821")
            }
            Self::ThreeHundredSeventyFiveCd0Db3Bbe4B7980F3954Ccf04F3D1 => {
                write!(f, "375cd0db-3bbe-4b79-80f3-954ccf04f3d1")
            }
            Self::UndefinedF143194C88994838A5184483B6 => {
                write!(f, "578584f1-4319-4c88-9948-38a5184483b6")
            }
            Self::D79B30274Eb245218722825Acfee7D8B => {
                write!(f, "d79b3027-4eb2-4521-8722-825acfee7d8b")
            }
            Self::TwoHundredFiftyTwoA082540B94B98Be807658956F13E9 => {
                write!(f, "252a0825-40b9-4b98-be80-7658956f13e9")
            }
            Self::SeventeenAa61C51C61477Ea40Ae52C8Ccd74B9 => {
                write!(f, "17aa61c5-1c61-477e-a40a-e52c8ccd74b9")
            }
            Self::B23Cd6E091Cd4A8A9869B30366Bf3966 => {
                write!(f, "b23cd6e0-91cd-4a8a-9869-b30366bf3966")
            }
            Self::SevenHundredThirtyOneEb2BeA74F4070B79735Bf7009E553 => {
                write!(f, "731eb2be-a74f-4070-b797-35bf7009e553")
            }
            Self::Ff86D4327F2747F8B02Fb5C102Ef6A55 => {
                write!(f, "ff86d432-7f27-47f8-b02f-b5c102ef6a55")
            }
            Self::ZeroF9Acbf693B542C7A227Fc7652755F65 => {
                write!(f, "0f9acbf6-93b5-42c7-a227-fc7652755f65")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
