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
        write!(f, "{}", match self {
            Self::Solid => "divide-solid",
            Self::Dotted => "divide-dotted",
            Self::Dashed => "divide-dashed",
            Self::Double => "divide-double",
            Self::Groove => "divide-groove",
            Self::Hidden => "divide-hidden",
            Self::None => "divide-none",
        })
    }
}
