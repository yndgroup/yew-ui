use std::fmt;
use yew::prelude::*;

use super::prelude::SetBorderWidth;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Border {
   pub width: SetBorderWidth,
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.width.clone().to_string(),)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::theme::prelude::{Border, BorderBase, BorderBottom, BorderLeft, BorderRight, BorderTop, BorderWidth, SetBorderWidth};

    #[test]
    fn test_border_display() {
        let border = Border {
            width: SetBorderWidth::Set(BorderWidth{
                border: BorderBase::V2,
                border_top: BorderTop::Default,
                border_right: BorderRight::Default,
                border_bottom: BorderBottom::Default,
                border_left: BorderLeft::Default,
            }),
        };
        // let border = Border {
        //     width: SetBorderWidth::BorderBase,
        // };
        println!("{:?}", border.to_string())
    }
}
