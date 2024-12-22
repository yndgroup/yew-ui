use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundRepeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
    RepeatRound,
    RepeatSpace,
    Default,
}

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bg-{}",
            match self {
                Self::Repeat => "repeat",
                Self::NoRepeat => "no-repeat",
                Self::RepeatX => "repeat-x",
                Self::RepeatY => "repeat-y",
                Self::RepeatRound => "repeat-round",
                Self::RepeatSpace => "repeat-space",
                Self::Default => "",
            }
        )
    }
}
