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
