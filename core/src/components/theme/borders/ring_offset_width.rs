use std::fmt;


#[derive(Clone, Debug, PartialEq)]
pub enum RingOffsetWidth {
    V0,
    V1,
    V2,
    V4,
    V8,
}

impl fmt::Display for RingOffsetWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ring-offset{}", match self {
            Self::V0 => "-0",
            Self::V1 => "-1",
            Self::V2 => "-2",
            Self::V4 => "-4",
            Self::V8 => "-8",
        })
    }
}