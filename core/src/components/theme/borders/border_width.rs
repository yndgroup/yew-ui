use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum BorderWidthEnum {
    V0,
    V2,
    V4,
    V8,
}

impl fmt::Display for BorderWidthEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BorderWidthEnum::V0 => "0",
                BorderWidthEnum::V2 => "2",
                BorderWidthEnum::V4 => "4",
                BorderWidthEnum::V8 => "8",
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum BorderWidth {
    V0,
    V2,
    V4,
    V8,
    Border,
    X(BorderWidthEnum),
    Y(BorderWidthEnum),
    InlineStart(BorderWidthEnum),
    InlineEnd(BorderWidthEnum),
    Top(BorderWidthEnum),
    Right(BorderWidthEnum),
    Bottom(BorderWidthEnum),
    Left(BorderWidthEnum),
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "border{}",
            match self {
                Self::V0 => "-0".to_string(),
                Self::V2 => "-2".to_string(),
                Self::V4 => "-4".to_string(),
                Self::V8 => "-8".to_string(),
                Self::Border => "".to_string(),
                Self::X(border_width_enum) =>
                    format!("-x-{}", border_width_enum.clone().to_string()),
                Self::Y(border_width_enum) =>
                    format!("-y-{}", border_width_enum.clone().to_string()),
                Self::InlineStart(border_width_enum) =>
                    format!("-s-{}", border_width_enum.clone().to_string()),
                Self::InlineEnd(border_width_enum) =>
                    format!("-e-{}", border_width_enum.clone().to_string()),
                Self::Top(border_width_enum) =>
                    format!("-t-{}", border_width_enum.clone().to_string()),
                Self::Right(border_width_enum) =>
                    format!("-r-{}", border_width_enum.clone().to_string()),
                Self::Bottom(border_width_enum) =>
                    format!("-b-{}", border_width_enum.clone().to_string()),
                Self::Left(border_width_enum) =>
                    format!("-l-{}", border_width_enum.clone().to_string()),
            }
        )
    }
}