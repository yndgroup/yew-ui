use std::fmt;


#[derive(Clone, Debug, PartialEq, Default)]
pub enum OutlineStyle {
    #[default]
    None,
    Outline,
    Dashed,
    Dotted,
    Double,
}

impl fmt::Display for OutlineStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "outline{}", match self {
            Self::None => "-none",
            Self::Outline => "",
            Self::Dashed => "-dashed",
            Self::Dotted => "-dotted",
            Self::Double => "-double",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outline_style_display() {
        assert_eq!(OutlineStyle::None.to_string(), "outline-none");
        assert_eq!(OutlineStyle::Outline.to_string(), "outline");
        assert_eq!(OutlineStyle::Dashed.to_string(), "outline-dashed");
        assert_eq!(OutlineStyle::Dotted.to_string(), "outline-dotted");
    }
}
