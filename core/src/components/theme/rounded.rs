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
