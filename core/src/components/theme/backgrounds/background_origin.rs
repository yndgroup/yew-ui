use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundOrigin {
    Border,
    Padding,
    Content,
    Default
}

impl fmt::Display for BackgroundOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bg-origin-{}", match self {
            Self::Border => "border",
            Self::Padding => "padding",
            Self::Content => "content",
            Self::Default => ""
        })
    }
}