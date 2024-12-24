use std::fmt;

#[derive(PartialEq, Debug)]
pub enum TypeEnum {
    Primary,
    Warning,
    Danger,
    Success,
    Info,
    Default,
}

impl Default for TypeEnum {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for TypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Primary => "primary",
                Self::Warning => "warning",
                Self::Danger => "danger",
                Self::Success => "success",
                Self::Info => "info",
                Self::Default => "default",
            }
        )
    }
}