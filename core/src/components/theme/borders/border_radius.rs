use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RadiusEnum {
    None,
    Sm,
    Rounded,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Full,
}

impl fmt::Display for RadiusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::None => "",
            Self::Sm => "sm",
            Self::Rounded => "rounded",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xl2 => "2xl",
            Self::Xl3 => "3xl",
            Self::Full => "full",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderRadius {
    Rounded(RadiusEnum),
    S(RadiusEnum),
    E(RadiusEnum),
    T(RadiusEnum),
    R(RadiusEnum),
    B(RadiusEnum),
    L(RadiusEnum),
    SS(RadiusEnum),
    SE(RadiusEnum),
    EE(RadiusEnum),
    ES(RadiusEnum),
    TL(RadiusEnum),
    TR(RadiusEnum),
    BR(RadiusEnum),
    BL(RadiusEnum),
}

/// https://tailwindcss.com/docs/border-radius
impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rounded{}", match self {
            Self::Rounded(rounded) => rounded.to_string(),
            Self::S(rounded) => format!("-s-{}", rounded.to_string()),
            Self::E(rounded) => format!("-e-{}", rounded.to_string()),
            Self::T(rounded) => format!("-t-{}", rounded.to_string()),
            Self::R(rounded) => format!("-r-{}", rounded.to_string()),
            Self::B(rounded) => format!("-b-{}", rounded.to_string()),
            Self::L(rounded) => format!("-l-{}", rounded.to_string()),
            Self::SS(rounded) => format!("-ss-{}", rounded.to_string()),
            Self::SE(rounded) => format!("-se-{}", rounded.to_string()),
            Self::EE(rounded) => format!("-ee-{}", rounded.to_string()),
            Self::ES(rounded) => format!("-es-{}", rounded.to_string()),
            Self::TL(rounded) => format!("-tl-{}", rounded.to_string()),
            Self::TR(rounded) => format!("-tr-{}", rounded.to_string()),
            Self::BR(rounded) => format!("-br-{}", rounded.to_string()),
            Self::BL(rounded) => format!("-bl-{}", rounded.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_display() {
        println!("{}", BorderRadius::Rounded(RadiusEnum::None));
        println!("{}", BorderRadius::E(RadiusEnum::Sm));
        println!("{}", BorderRadius::E(RadiusEnum::Md));
        println!("{}", BorderRadius::S(RadiusEnum::Full))
    }
}