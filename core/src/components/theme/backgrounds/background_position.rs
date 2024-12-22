use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundOrigin {
    Bottom,
    Center,
    Left,
    LeftBottom,
    Right,
    RightBottom,
    RightTop,
    Top,
    Default,
}

impl fmt::Display for BackgroundOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bg-{}", match self {
            Self::Bottom => "bottom",
            Self::Center => "center",
            Self::Left => "left",
            Self::LeftBottom => "left-bottom",
            Self::Right => "right",
            Self::RightBottom => "right-bottom",
            Self::RightTop => "right-top",
            Self::Top => "top",
            Self::Default => "",
        })
    }
}