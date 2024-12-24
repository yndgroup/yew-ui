use std::fmt;

use crate::components::theme::ColorEnum;

#[derive(Clone, Debug, PartialEq)]
pub enum BorderColor {
    Inherit(BorderPosition),
    Current(BorderPosition),
    Transparent(BorderPosition),
    Black(BorderPosition),
    White(BorderPosition),
    Slate(ColorEnum, BorderPosition),
    Gray(ColorEnum, BorderPosition),
    Zinc(ColorEnum, BorderPosition),
    Neutral(ColorEnum, BorderPosition),
    Stone(ColorEnum, BorderPosition),
    Red(ColorEnum, BorderPosition),
    Orange(ColorEnum, BorderPosition),
    Amber(ColorEnum, BorderPosition),
    Yellow(ColorEnum, BorderPosition),
    Lime(ColorEnum, BorderPosition),
    Green(ColorEnum, BorderPosition),
    Emerald(ColorEnum, BorderPosition),
    Teal(ColorEnum, BorderPosition),
    Cyan(ColorEnum, BorderPosition),
    Sky(ColorEnum, BorderPosition),
    Blue(ColorEnum, BorderPosition),
    Indigo(ColorEnum, BorderPosition),
    Violet(ColorEnum, BorderPosition),
    Purple(ColorEnum, BorderPosition),
    Fuchsia(ColorEnum, BorderPosition),
    Pink(ColorEnum, BorderPosition),
    Rose(ColorEnum, BorderPosition),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderPosition {
    X,
    Y,
    InlineStart,
    InlineEnd,
    Top,
    Right,
    Bottom,
    Left,
    All,
}

impl fmt::Display for BorderPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "x".to_string(),
                Self::Y => "y".to_string(),
                Self::InlineStart => "s".to_string(),
                Self::InlineEnd => "e".to_string(),
                Self::Top => "t".to_string(),
                Self::Right => "r".to_string(),
                Self::Bottom => "b".to_string(),
                Self::Left => "l".to_string(),
                Self::All => "".to_string(),
            }
        )
    }
}

impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "border-{}",
            match self {
                BorderColor::Inherit(bp) => match bp {
                    BorderPosition::All => "inherit".to_string(),
                    _ => format!("{}-current", bp),
                },
                BorderColor::Current(bp) => match bp {
                    BorderPosition::All => "current".to_string(),
                    _ => format!("{}-current", bp),
                },
                BorderColor::Transparent(bp) => match bp {
                    BorderPosition::All => "transparent".to_string(),
                    _ => format!("{}-transparent", bp),
                },
                BorderColor::Black(bp) => match bp {
                    BorderPosition::All => "black".to_string(),
                    _ => format!("{}-black", bp),
                },
                BorderColor::White(bp) => match bp {
                    BorderPosition::All => "white".to_string(),
                    _ => format!("{}-white", bp),
                },
                BorderColor::Slate(cv, bp) => match bp {
                    BorderPosition::All => format!("slate-{}", cv),
                    _ => format!("{}-slate-{}", bp, cv),
                },
                BorderColor::Gray(cv, bp) => match bp {
                    BorderPosition::All => format!("gray-{}", cv),
                    _ => format!("{}-gray-{}", bp, cv),
                },
                BorderColor::Zinc(cv, bp) => match bp {
                    BorderPosition::All => format!("zinc-{}", cv),
                    _ => format!("{}-zinc-{}", bp, cv),
                },
                BorderColor::Neutral(cv, bp) => match bp {
                    BorderPosition::All => format!("neutral-{}", cv),
                    _ => format!("{}-neutral-{}", bp, cv),
                },
                BorderColor::Stone(cv, bp) => match bp {
                    BorderPosition::All => format!("stone-{}", cv),
                    _ => format!("{}-stone-{}", bp, cv),
                },
                BorderColor::Red(cv, bp) => match bp {
                    BorderPosition::All => format!("red-{}", cv),
                    _ => format!("{}-red-{}", bp, cv),
                },
                BorderColor::Orange(cv, bp) => match bp {
                    BorderPosition::All => format!("orange-{}", cv),
                    _ => format!("{}-orange-{}", bp, cv),
                },
                BorderColor::Amber(cv, bp) => match bp {
                    BorderPosition::All => format!("amber-{}", cv),
                    _ => format!("{}-amber-{}", bp, cv),
                },
                BorderColor::Yellow(cv, bp) => match bp {
                    BorderPosition::All => format!("yellow-{}", cv),
                    _ => format!("{}-yellow-{}", bp, cv),
                },
                BorderColor::Lime(cv, bp) => match bp {
                    BorderPosition::All => format!("lime-{}", cv),
                    _ => format!("{}-lime-{}", bp, cv),
                },
                BorderColor::Green(cv, bp) => match bp {
                    BorderPosition::All => format!("green-{}", cv),
                    _ => format!("{}-green-{}", bp, cv),
                },
                BorderColor::Emerald(cv, bp) => match bp {
                    BorderPosition::All => format!("emerald-{}", cv),
                    _ => format!("{}-emerald-{}", bp, cv),
                },
                BorderColor::Teal(cv, bp) => match bp {
                    BorderPosition::All => format!("teal-{}", cv),
                    _ => format!("{}-teal-{}", bp, cv),
                },
                BorderColor::Cyan(cv, bp) => match bp {
                    BorderPosition::All => format!("cyan-{}", cv),
                    _ => format!("{}-cyan-{}", bp, cv),
                },
                BorderColor::Sky(cv, bp) => match bp {
                    BorderPosition::All => format!("sky-{}", cv),
                    _ => format!("{}-sky-{}", bp, cv),
                },
                BorderColor::Blue(cv, bp) => match bp {
                    BorderPosition::All => format!("blue-{}", cv),
                    _ => format!("{}-blue-{}", bp, cv),
                },
                BorderColor::Indigo(cv, bp) => match bp {
                    BorderPosition::All => format!("indigo-{}", cv),
                    _ => format!("{}-indigo-{}", bp, cv),
                },
                BorderColor::Violet(cv, bp) => match bp {
                    BorderPosition::All => format!("violet-{}", cv),
                    _ => format!("{}-violet-{}", bp, cv),
                },
                BorderColor::Purple(cv, bp) => match bp {
                    BorderPosition::All => format!("purple-{}", cv),
                    _ => format!("{}-purple-{}", bp, cv),
                },
                BorderColor::Fuchsia(cv, bp) => match bp {
                    BorderPosition::All => format!("fuchsia-{}", cv),
                    _ => format!("{}-fuchsia-{}", bp, cv),
                },
                BorderColor::Pink(cv, bp) => match bp {
                    BorderPosition::All => format!("pink-{}", cv),
                    _ => format!("{}-pink-{}", bp, cv),
                },
                BorderColor::Rose(cv, bp) => match bp {
                    BorderPosition::All => format!("rose-{}", cv),
                    _ => format!("{}-rose-{}", bp, cv),
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::components::theme::ColorEnum;

    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!("border-gray-100", BorderColor::Gray(ColorEnum::V100, BorderPosition::All).to_string());
        assert_eq!("border-inherit", BorderColor::Inherit(BorderPosition::All).to_string());
        assert_eq!("border-x-gray-100", BorderColor::Gray(ColorEnum::V100, BorderPosition::X).to_string());
        assert_eq!("border-y-gray-100", BorderColor::Gray(ColorEnum::V100, BorderPosition::Y).to_string());
    }
}
