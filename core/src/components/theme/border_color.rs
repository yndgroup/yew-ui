use super::ColorLevel;

pub enum BorderColor {
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