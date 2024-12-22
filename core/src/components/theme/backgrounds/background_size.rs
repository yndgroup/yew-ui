use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundRepeat {
    Auto,
    Cover,
    Contain,
    Default,
}

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bg-{}",
            match self {
                Self::Auto => "auto",
                Self::Cover => "cover",
                Self::Contain => "contain",
                Self::Default => "",
            }
        )
    }
}
