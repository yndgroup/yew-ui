use std::fmt::{self};

#[derive(PartialEq, Clone, Debug)]
pub enum DivideLevel {
    V0,
    V2,
    V4,
    V8,
    Reverse,
    Default,
}

impl fmt::Display for DivideLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DivideLevel::V0 => "-0",
            DivideLevel::V2 => "-2",
            DivideLevel::V4 => "-4",
            DivideLevel::V8 => "-8",
            DivideLevel::Reverse => "-reverse",
            DivideLevel::Default => "",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum  DivideWidth {
    X(DivideLevel),
    Y(DivideLevel),
}

impl fmt::Display for DivideWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DivideWidth::X(level) => format!("divide-x{}", level),
            DivideWidth::Y(level) => format!("divide-y{}", level),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_divide_display() {
        let x_v0 = DivideWidth::X(DivideLevel::V0);
        let y_v0 = DivideWidth::Y(DivideLevel::V0);
        assert_eq!("divide-x-0", x_v0.to_string());
        assert_eq!("divide-y-0", y_v0.to_string());

        let x_reverse = DivideWidth::X(DivideLevel::Reverse);
        let y_reverse = DivideWidth::Y(DivideLevel::Reverse);
        assert_eq!("divide-x-reverse", x_reverse.to_string());
        assert_eq!("divide-y-reverse", y_reverse.to_string());

        let d_x = DivideWidth::X(DivideLevel::Default);
        let d_y = DivideWidth::Y(DivideLevel::Default);
        println!("divide-x => {}", d_x);
        assert_eq!("divide-x", d_x.to_string());
        assert_eq!("divide-y", d_y.to_string());
    }
}
