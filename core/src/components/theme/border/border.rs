
use yew::Properties;
use std::fmt;

use super::{BorderColor, BorderStyle, DivideColor, DivideStyle, DivideWidth, SetBorderWidth};


#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Border {
   pub width: SetBorderWidth,
   pub color: BorderColor,
   pub style: BorderStyle,
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.width.clone().to_string(), self.color.clone().to_string(), self.style.clone().to_string())
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Divide {
    pub width: DivideWidth,
    pub color: DivideColor,
    pub style: DivideStyle,
}

impl fmt::Display for Divide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.width.clone().to_string(), self.color.clone().to_string(), self.style.clone().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::components::theme::{prelude::{Border, BorderBase, BorderBottom, BorderLeft, BorderRight, BorderTop, BorderWidth, SetBorderWidth}, BorderColor, BorderPosition, BorderStyle, ColorLevel, Divide, DivideColor, DivideLevel, DivideStyle, DivideWidth};

    #[test]
    fn test_divide_display() {
        let divide = Divide {
            width: DivideWidth::Y(DivideLevel::Reverse),
            color: DivideColor::Violet(ColorLevel::N100),
            style: DivideStyle::Solid,
        };
        println!("{:?}", divide.to_string())
    }

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
            color: BorderColor::Black(BorderPosition::Default),
            style: BorderStyle::Dashed,
        };
        // let border = Border {
        //     width: SetBorderWidth::BorderBase,
        // };
        println!("{:?}", border.to_string())
    }
}
