use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
    Default
}

impl fmt::Display for BackgroundAttachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bg-{}", match self {
            Self::Fixed => "fixed",
            Self::Local => "local",
            Self::Scroll => "scroll",
            Self::Default => ""
        })
        /* match self {
            Self::Fixed => write!(f, "bg-fixed"),
            Self::Local => write!(f, "bg-local"),
            Self::Scroll => write!(f, "bg-scroll"),
            Self::Default => write!(f, "bg-default"),
        } */
    }
}