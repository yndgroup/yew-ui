use std::fmt;

use crate::components::theme::ColorLevel;

#[derive(Clone, Debug, PartialEq)]
pub enum DivideColor {
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

impl fmt::Display for DivideColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Inherit => "divide-inherit".to_string(),
                Self::Current => "divide-current".to_string(),
                Self::Transparent => "divide-transparent".to_string(),
                Self::Black => "divide-black".to_string(),
                Self::White => "divide-white".to_string(),
                Self::Slate(level) => format!("divide-slate-{}", level),
                Self::Gray(level) => format!("divide-gray-{}", level),
                Self::Zinc(level) => format!("divide-zinc-{}", level),
                Self::Neutral(level) => format!("divide-neutral-{}", level),
                Self::Stone(level) => format!("divide-stone-{}", level),
                Self::Red(level) => format!("divide-red-{}", level),
                Self::Orange(level) => format!("divide-orange-{}", level),
                Self::Amber(level) => format!("divide-amber-{}", level),
                Self::Yellow(level) => format!("divide-yellow-{}", level),
                Self::Lime(level) => format!("divide-lime-{}", level),
                Self::Green(level) => format!("divide-green-{}", level),
                Self::Emerald(level) => format!("divide-emerald-{}", level),
                Self::Teal(level) => format!("divide-teal-{}", level),
                Self::Cyan(level) => format!("divide-cyan-{}", level),
                Self::Sky(level) => format!("divide-sky-{}", level),
                Self::Blue(level) => format!("divide-blue-{}", level),
                Self::Indigo(level) => format!("divide-indigo-{}", level),
                Self::Violet(level) => format!("divide-violet-{}", level),
                Self::Purple(level) => format!("divide-purple-{}", level),
                Self::Fuchsia(level) => format!("divide-fuchsia-{}", level),
                Self::Pink(level) => format!("divide-pink-{}", level),
                Self::Rose(level) => format!("divide-rose-{}", level),
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_divide_color_display() {
        assert_eq!("divide-gray-100", DivideColor::Gray(ColorLevel::N100).to_string());
        assert_eq!("divide-red-100", DivideColor::Red(ColorLevel::N100).to_string());
        assert_eq!("divide-teal-100", DivideColor::Teal(ColorLevel::N100).to_string());
    }
}
