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
            BorderStyle::Solid => "border-solid",
            BorderStyle::Dotted => "border-dotted",
            BorderStyle::Dashed => "border-dashed",
            BorderStyle::Double => "border-double",
            BorderStyle::Groove => "border-groove",
            BorderStyle::Hidden => "border-hidden",
            BorderStyle::None => "border-none",
        })
    }
}
