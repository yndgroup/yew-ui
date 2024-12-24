use super::{BorderColor, BorderStyle, BorderWidth, ColorEnum, Colors, TypeEnum};

#[derive(Clone, Debug)]
pub struct Class {
    class_name: Vec<String>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            class_name: Vec::new(),
        }
    }

    /// set type
    pub fn set_type(&mut self, type_name: TypeEnum) -> &mut Self {
        self.class_name.push(type_name.to_string());
        self
    }

    /// set color value
    pub fn set_color(&mut self, colors: Colors) -> &mut Self {
        self.class_name.push(colors.to_string());
        self
    }

    /// set border color value
    pub fn set_border_color(&mut self, border_color: BorderColor) -> &mut Self {
        self.class_name.push(border_color.to_string());
        self
    }

    /// set border width
    pub fn set_border_style(&mut self, border_style: BorderStyle) -> &mut Self {
        self.class_name.push(border_style.to_string());
        self
    }

    /// get class name
    pub fn get_class_name(&self) -> String {
        self.class_name.join(" ")
    }
}


#[cfg(test)]
mod tests {
    use crate::components::theme::BorderPosition;

    use super::*;
    #[test]
    fn test_get_type() {
        let mut class = Class::new();
        class
            .set_type(TypeEnum::Primary)
            .set_color(Colors::Blue(ColorEnum::V100))
            .set_border_color(BorderColor::Black(BorderPosition::X))
            .set_border_color(BorderColor::Slate(ColorEnum::V100, BorderPosition::X))
            .set_border_style(BorderStyle::Dashed)
            ;
        println!("class = {}", class.get_class_name())
    }
}