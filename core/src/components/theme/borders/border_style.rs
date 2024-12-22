use std::fmt;


#[derive(Clone, Debug, PartialEq, Default)]
pub enum BorderStyle {
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Hidden,
    #[default]
    None,
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Solid => "border-solid",
            Self::Dotted => "border-dotted",
            Self::Dashed => "border-dashed",
            Self::Double => "border-double",
            Self::Groove => "border-groove",
            Self::Hidden => "border-hidden",
            Self::None => "border-none",
        })
    }
}
