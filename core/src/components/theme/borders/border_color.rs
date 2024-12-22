use std::fmt;

use crate::components::theme::ColorLevel;

#[derive(Clone, Debug, PartialEq)]
pub enum BorderColor {
    Inherit(BorderPosition),
    Current(BorderPosition),
    Transparent(BorderPosition),
    Black(BorderPosition),
    White(BorderPosition),
    Slate(ColorLevel, BorderPosition),
    Gray(ColorLevel, BorderPosition),
    Zinc(ColorLevel, BorderPosition),
    Neutral(ColorLevel, BorderPosition),
    Stone(ColorLevel, BorderPosition),
    Red(ColorLevel, BorderPosition),
    Orange(ColorLevel, BorderPosition),
    Amber(ColorLevel, BorderPosition),
    Yellow(ColorLevel, BorderPosition),
    Lime(ColorLevel, BorderPosition),
    Green(ColorLevel, BorderPosition),
    Emerald(ColorLevel, BorderPosition),
    Teal(ColorLevel, BorderPosition),
    Cyan(ColorLevel, BorderPosition),
    Sky(ColorLevel, BorderPosition),
    Blue(ColorLevel, BorderPosition),
    Indigo(ColorLevel, BorderPosition),
    Violet(ColorLevel, BorderPosition),
    Purple(ColorLevel, BorderPosition),
    Fuchsia(ColorLevel, BorderPosition),
    Pink(ColorLevel, BorderPosition),
    Rose(ColorLevel, BorderPosition),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderPosition {
    X,
    Y,
    S,
    E,
    T,
    R,
    B,
    L,
    Default,
}

impl fmt::Display for BorderPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "x".to_string(),
                Self::Y => "y".to_string(),
                Self::S => "s".to_string(),
                Self::E => "e".to_string(),
                Self::T => "t".to_string(),
                Self::R => "r".to_string(),
                Self::B => "b".to_string(),
                Self::L => "l".to_string(),
                Self::Default => "".to_string(),
            }
        )
    }
}

impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BorderColor::Inherit(bp) => match bp {
                    BorderPosition::Default => "border-inherit".to_string(),
                    _ => format!("border-{}-current", bp),
                },
                BorderColor::Current(bp) => match bp {
                    BorderPosition::Default => "border-current".to_string(),
                    _ => format!("border-{}-current", bp),
                },
                BorderColor::Transparent(bp) => match bp {
                    BorderPosition::Default => "border-transparent".to_string(),
                    _ => format!("border-{}-transparent", bp),
                },
                BorderColor::Black(bp) => match bp {
                    BorderPosition::Default => "border-black".to_string(),
                    _ => format!("border-{}-black", bp),
                },
                BorderColor::White(bp) => match bp {
                    BorderPosition::Default => "border-white".to_string(),
                    _ => format!("border-{}-white", bp),
                },
                BorderColor::Slate(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-slate-{}", cv),
                    _ => format!("border-{}-slate-{}", bp, cv),
                },
                BorderColor::Gray(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-gray-{}", cv),
                    _ => format!("border-{}-gray-{}", bp, cv),
                },
                BorderColor::Zinc(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-zinc-{}", cv),
                    _ => format!("border-{}-zinc-{}", bp, cv),
                },
                BorderColor::Neutral(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-neutral-{}", cv),
                    _ => format!("border-{}-neutral-{}", bp, cv),
                },
                BorderColor::Stone(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-stone-{}", cv),
                    _ => format!("border-{}-stone-{}", bp, cv),
                },
                BorderColor::Red(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-red-{}", cv),
                    _ => format!("border-{}-red-{}", bp, cv),
                },
                BorderColor::Orange(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-orange-{}", cv),
                    _ => format!("border-{}-orange-{}", bp, cv),
                },
                BorderColor::Amber(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-amber-{}", cv),
                    _ => format!("border-{}-amber-{}", bp, cv),
                },
                BorderColor::Yellow(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-yellow-{}", cv),
                    _ => format!("border-{}-yellow-{}", bp, cv),
                },
                BorderColor::Lime(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-lime-{}", cv),
                    _ => format!("border-{}-lime-{}", bp, cv),
                },
                BorderColor::Green(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-green-{}", cv),
                    _ => format!("border-{}-green-{}", bp, cv),
                },
                BorderColor::Emerald(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-emerald-{}", cv),
                    _ => format!("border-{}-emerald-{}", bp, cv),
                },
                BorderColor::Teal(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-teal-{}", cv),
                    _ => format!("border-{}-teal-{}", bp, cv),
                },
                BorderColor::Cyan(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-cyan-{}", cv),
                    _ => format!("border-{}-cyan-{}", bp, cv),
                },
                BorderColor::Sky(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-sky-{}", cv),
                    _ => format!("border-{}-sky-{}", bp, cv),
                },
                BorderColor::Blue(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-blue-{}", cv),
                    _ => format!("border-{}-blue-{}", bp, cv),
                },
                BorderColor::Indigo(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-indigo-{}", cv),
                    _ => format!("border-{}-indigo-{}", bp, cv),
                },
                BorderColor::Violet(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-violet-{}", cv),
                    _ => format!("border-{}-violet-{}", bp, cv),
                },
                BorderColor::Purple(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-purple-{}", cv),
                    _ => format!("border-{}-purple-{}", bp, cv),
                },
                BorderColor::Fuchsia(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-fuchsia-{}", cv),
                    _ => format!("border-{}-fuchsia-{}", bp, cv),
                },
                BorderColor::Pink(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-pink-{}", cv),
                    _ => format!("border-{}-pink-{}", bp, cv),
                },
                BorderColor::Rose(cv, bp) => match bp {
                    BorderPosition::Default => format!("border-rose-{}", cv),
                    _ => format!("border-{}-rose-{}", bp, cv),
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!("border-gray-100", BorderColor::Gray(ColorLevel::N100, BorderPosition::Default).to_string());
        assert_eq!("border-x-gray-100", BorderColor::Gray(ColorLevel::N100, BorderPosition::X).to_string());
        assert_eq!("border-y-gray-100", BorderColor::Gray(ColorLevel::N100, BorderPosition::Y).to_string());
    }
}
