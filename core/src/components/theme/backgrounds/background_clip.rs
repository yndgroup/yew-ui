use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text
}

impl fmt::Display for BackgroundClip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bg-clip-{}", match self {
            Self::Border => "border",
            Self::Padding => "padding",
            Self::Content => "content",
            Self::Text => "text"
        })
    }
}