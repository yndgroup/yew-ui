use std::fmt;

use crate::components::theme::ColorLevel;

#[derive(Clone, Debug, PartialEq)]
pub enum OutlineColor {
    Inherit,
    Current,
    Transparent,
    Black,
    White,
    Slate(ColorLevel),
    Gray(ColorLevel),
    Zinc(ColorLevel),
    Neutral(ColorLevel),
    Stone(ColorLevel),
    Red(ColorLevel),
    Orange(ColorLevel),
    Amber(ColorLevel),
    Yellow(ColorLevel),
    Lime(ColorLevel),
    Green(ColorLevel),
    Emerald(ColorLevel),
    Teal(ColorLevel),
    Cyan(ColorLevel),
    Sky(ColorLevel),
    Blue(ColorLevel),
    Indigo(ColorLevel),
    Violet(ColorLevel),
    Purple(ColorLevel),
    Fuchsia(ColorLevel),
    Pink(ColorLevel),
    Rose(ColorLevel),
}

impl fmt::Display for OutlineColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "outline-{}",
            match self {
                Self::Inherit => "inherit".to_string(),
                Self::Current => "current".to_string(),
                Self::Transparent => "transparent".to_string(),
                Self::Black => "black".to_string(),
                Self::White => "white".to_string(),
                Self::Slate(level) => format!("slate-{}", level),
                Self::Gray(level) => format!("gray-{}", level),
                Self::Zinc(level) => format!("zinc-{}", level),
                Self::Neutral(level) => format!("neutral-{}", level),
                Self::Stone(level) => format!("stone-{}", level),
                Self::Red(level) => format!("red-{}", level),
                Self::Orange(level) => format!("orange-{}", level),
                Self::Amber(level) => format!("amber-{}", level),
                Self::Yellow(level) => format!("yellow-{}", level),
                Self::Lime(level) => format!("lime-{}", level),
                Self::Green(level) => format!("green-{}", level),
                Self::Emerald(level) => format!("emerald-{}", level),
                Self::Teal(level) => format!("teal-{}", level),
                Self::Cyan(level) => format!("cyan-{}", level),
                Self::Sky(level) => format!("sky-{}", level),
                Self::Blue(level) => format!("blue-{}", level),
                Self::Indigo(level) => format!("indigo-{}", level),
                Self::Violet(level) => format!("violet-{}", level),
                Self::Purple(level) => format!("purple-{}", level),
                Self::Fuchsia(level) => format!("fuchsia-{}", level),
                Self::Pink(level) => format!("pink-{}", level),
                Self::Rose(level) => format!("rose-{}", level),
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_outline_color_display() {
        assert_eq!("outline-gray-100", OutlineColor::Gray(ColorLevel::N100).to_string());
        assert_eq!("outline-red-100", OutlineColor::Red(ColorLevel::N100).to_string());
        assert_eq!("outline-teal-100", OutlineColor::Teal(ColorLevel::N100).to_string());
    }
}
