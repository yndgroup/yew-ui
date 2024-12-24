#[derive(PartialEq, Debug)]
pub enum SizeStyle {
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
    Xl9,
}

impl Default for SizeStyle {
    fn default() -> Self {
        Self::Base
    }
}