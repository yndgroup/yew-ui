use std::fmt;


#[derive(Clone, Debug, PartialEq, Default)]
pub enum DivideStyle {
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Hidden,
    #[default]
    None,
}

impl fmt::Display for DivideStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "divide-{}", match self {
            Self::Solid => "solid",
            Self::Dotted => "dotted",
            Self::Dashed => "dashed",
            Self::Double => "double",
            Self::Groove => "groove",
            Self::Hidden => "hidden",
            Self::None => "none",
        })
    }
}
