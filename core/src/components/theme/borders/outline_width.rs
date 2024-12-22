use std::fmt;


#[derive(Clone, Debug, PartialEq)]
pub enum OutlineWidth {
    V0,
    V1,
    V2,
    V4,
    V8
}

impl fmt::Display for OutlineWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "outline-{}", match self {
            Self::V0 => "0",
            Self::V1 => "1",
            Self::V2 => "2",
            Self::V4 => "4",
            Self::V8 => "8",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outline_width_display() {
        println!("{}", OutlineWidth::V8);
        assert_eq!("outline-0", OutlineWidth::V0.to_string());
        assert_eq!("outline-1", OutlineWidth::V1.to_string());
        assert_eq!("outline-2", OutlineWidth::V2.to_string());
    }
}