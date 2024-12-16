#[derive(PartialEq, Debug)]
pub enum TypeStyle {
    Primary,
    Warning,
    Danger,
    Success,
    Info,
    Default,
}

impl Default for TypeStyle {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(PartialEq, Debug)]
pub enum  SizeStyle {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
    Xl7,
    Xl8,
    Xl9
}

impl Default for SizeStyle {
    fn default() -> Self {
        Self::Base
    }
}

#[derive(PartialEq, Debug)]
pub enum Rounded {
    None,
    Sm,
    Rounded,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Full,
}

impl Default for Rounded {
    fn default() -> Self {
        Self::None
    }
}