use std::fmt::{self};

use yew::Properties;

#[derive(Clone, Debug, PartialEq)]
pub enum SetBorderWidth {
    Set(BorderWidth),
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    BorderBase,
    Default,
}

impl fmt::Display for SetBorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SetBorderWidth::Set(border_width) => border_width.to_string(),
                SetBorderWidth::BorderTop => BorderTop::default().to_string(),
                SetBorderWidth::BorderRight => BorderRight::Default.to_string(),
                SetBorderWidth::BorderBottom => BorderBottom::Default.to_string(),
                SetBorderWidth::BorderLeft => BorderLeft::Default.to_string(),
                SetBorderWidth::BorderBase => BorderBase::Default.to_string(),
                SetBorderWidth::Default => "".to_string(),
            }
        )
    }
}

impl Default for SetBorderWidth {
    fn default() -> Self {
        Self::Set(BorderWidth {
            border: BorderBase::Default,
            border_top: BorderTop::Default,
            border_right: BorderRight::Default,
            border_bottom: BorderBottom::Default,
            border_left: BorderLeft::Default,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct BorderWidth {
    pub border: BorderBase,
    pub border_top: BorderTop,
    pub border_right: BorderRight,
    pub border_bottom: BorderBottom,
    pub border_left: BorderLeft,
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.border, self.border_top, self.border_right, self.border_bottom, self.border_left
        )
    }
}

// class border
#[derive(Clone, Debug, Default, PartialEq)]
pub enum BorderBase {
    V0,
    V2,
    V4,
    V8,
    #[default]
    Default,
}

impl fmt::Display for BorderBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "border-0",
                Self::V2 => "border-2",
                Self::V4 => "border-4",
                Self::V8 => "border-8",
                Self::Default => "border",
            }
        )
    }
}

// class border-top
#[derive(Clone, Debug, Default, PartialEq)]
pub enum BorderTop {
    V0,
    V2,
    V4,
    V8,
    #[default]
    Default,
}

impl fmt::Display for BorderTop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "border-t-0",
                Self::V2 => "border-t-2",
                Self::V4 => "border-t-4",
                Self::V8 => "border-t-8",
                Self::Default => "border-t",
            }
        )
    }
}

// class border-right
#[derive(Clone, Debug, Default, PartialEq)]
pub enum BorderRight {
    V0,
    V2,
    V4,
    V8,
    #[default]
    Default,
}

impl fmt::Display for BorderRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "border-r-0",
                Self::V2 => "border-r-2",
                Self::V4 => "border-r-4",
                Self::V8 => "border-r-8",
                Self::Default => "border-r",
            }
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum BorderBottom {
    V0,
    V2,
    V4,
    V8,
    #[default]
    Default,
}

impl fmt::Display for BorderBottom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "border-b-0",
                Self::V2 => "border-b-2",
                Self::V4 => "border-b-4",
                Self::V8 => "border-b-8",
                Self::Default => "border-b",
            }
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum BorderLeft {
    V0,
    V2,
    V4,
    V8,
    #[default]
    Default,
}

impl fmt::Display for BorderLeft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "border-l-0",
                Self::V2 => "border-l-2",
                Self::V4 => "border-l-4",
                Self::V8 => "border-l-8",
                Self::Default => "border-l",
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_border_display() {
        let b = SetBorderWidth::default();
        println!("SetBorderWidth => {:?}", b.to_string());
    }

    #[test]
    fn test_border_display() {
        println!("Border::W0 => {}", BorderBase::V0);
        println!(
            "BorderWidth => {:?}",
            BorderWidth {
                border: Default::default(),
                border_top: Default::default(),
                border_right: Default::default(),
                border_bottom: Default::default(),
                border_left: BorderLeft::V2,
            }
            .to_string()
        );
        // assert_eq!(Border::W0.to_string(), "border-0");
    }
}
