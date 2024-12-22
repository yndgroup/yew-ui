use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BackgroundRepeat {
    GradientToT,
    GradientToTR,
    GradientToR,
    GradientToBR,
    GradientToB,
    GradientToBL,
    GradientToL,
    GradientToTL,
    Default,
}

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bg-{}",
            match self {
                Self::GradientToT => "gradient-to-t",
                Self::GradientToTR => "gradient-to-tr",
                Self::GradientToR => "gradient-to-r",
                Self::GradientToBR => "gradient-to-br",
                Self::GradientToB => "gradient-to-b",
                Self::GradientToBL => "gradient-to-bl",
                Self::GradientToL => "gradient-to-l",
                Self::GradientToTL => "gradient-to-tl",
                Self::Default => ""
            }
        )
    }
}
