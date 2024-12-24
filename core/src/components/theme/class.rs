use super::{BorderColor, BorderRadius, BorderStyle, BorderWidth, Colors, DivideColor, DivideStyle, DivideWidth, OutlineColor, OutlineOffset, OutlineStyle, OutlineWidth, RingColor, RingOffsetColor, RingOffsetWidth, RingWidth, TypeEnum};

#[derive(Clone, Debug)]
pub struct ClassBuilder {
    class_name: Vec<String>,
}

impl ClassBuilder {
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

    /// Utilities for controlling the color of an element's borders.
    pub fn set_border_color(&mut self, border_color: BorderColor) -> &mut Self {
        self.class_name.push(border_color.to_string());
        self
    }

    /// Utilities for controlling the style of an element's borders.
    pub fn set_border_style(&mut self, border_style: BorderStyle) -> &mut Self {
        self.class_name.push(border_style.to_string());
        self
    }

    pub fn set_border_width(&mut self, border_width: BorderWidth) -> &mut Self {
        self.class_name.push(border_width.to_string());
        self
    }

    /// Utilities for controlling the border width between elements.
    pub fn set_divide_width(&mut self, divide_width: DivideWidth) -> &mut Self {
        self.class_name.push(divide_width.to_string());
        self
    }

    /// Utilities for controlling the border color between elements.
    pub fn set_divide_color(&mut self, divide_color: DivideColor) -> &mut Self {
        self.class_name.push(divide_color.to_string());
        self
    }

    /// Utilities for controlling the border style between elements.
    pub fn set_divide_style(&mut self, divide_style: DivideStyle) -> &mut Self {
        self.class_name.push(divide_style.to_string());
        self
    }

    /// Utilities for controlling the color of an element's outline.
    pub fn set_outline_color(&mut self, outline_color: OutlineColor) -> &mut Self {
        self.class_name.push(outline_color.to_string());
        self
    }

    /// Utilities for controlling the width of an element's outline.
    pub fn set_outline_width(&mut self, outline_color: OutlineWidth) -> &mut Self {
        self.class_name.push(outline_color.to_string());
        self
    }

    /// Utilities for controlling the style of an element's outline.
    pub fn set_outline_style(&mut self, outline_style: OutlineStyle) -> &mut Self {
        self.class_name.push(outline_style.to_string());
        self
    }

    /// Utilities for simulating an offset when adding outline rings.
    pub fn set_outline_offset(&mut self, outline_offset: OutlineOffset) -> &mut Self {
        self.class_name.push(outline_offset.to_string());
        self
    }

    /// Utilities for setting the color of outline rings.
    pub fn set_ring_color(&mut self, ring_color: RingColor) -> &mut Self {
        self.class_name.push(ring_color.to_string());
        self
    }

    /// Utilities for creating outline rings with box-shadows.
    pub fn set_ring_width(&mut self, ring_width: RingWidth) -> &mut Self {
        self.class_name.push(ring_width.to_string());
        self
    }

    /// Utilities for setting the color of outline ring offsets.
    pub fn set_ring_offset_color(&mut self, ring_offset_color: RingOffsetColor) -> &mut Self {
        self.class_name.push(ring_offset_color.to_string());
        self
    }

    /// Utilities for simulating an offset when adding outline rings.
    pub fn set_ring_offset_width(&mut self, ring_offset_width: RingOffsetWidth) -> &mut Self {
        self.class_name.push(ring_offset_width.to_string());
        self
    }

    /// Utilities for controlling the border radius of an element.
    pub fn set_border_radius(&mut self, border_radius: BorderRadius) -> &mut Self {
        self.class_name.push(border_radius.to_string());
        self
    }

    /// get class name string
    pub fn get_class_name(&self) -> String {
        self.class_name.join(" ")
    }
}

#[cfg(test)]
mod tests {

    use crate::components::theme::RadiusEnum;

    use super::*;
    #[test]
    fn test_get_type() {
        let mut class = ClassBuilder::new();
        class
            /* .set_type(TypeEnum::Primary)
            .set_color(Colors::Blue(ColorEnum::V100)) */
            // border
           /*  .set_border_color(BorderColor::Black(BorderPosition::X))
            .set_border_color(BorderColor::Slate(ColorEnum::V100, BorderPosition::X))
            .set_border_style(BorderStyle::Dashed)
            .set_border_width(BorderWidth::Top(BorderWidthEnum::V8))
            .set_border_width(BorderWidth::Left(BorderWidthEnum::V8)) */
            // divide
            /* .set_divide_width(DivideWidth::X(DivideLevel::V2))
            .set_divide_color(DivideColor::Inherit)
            .set_divide_style(DivideStyle::Dotted) */
            // outline
           /*  .set_outline_width(OutlineWidth::V4)
            .set_outline_color(OutlineColor::Slate(ColorEnum::V100))
            .set_outline_style(OutlineStyle::Dotted)
            .set_outline_offset(OutlineOffset::V4) */
            .set_ring_color(RingColor::Black)
            .set_ring_width(RingWidth::V4)
            .set_ring_offset_color(RingOffsetColor::Black)
            .set_ring_offset_width(RingOffsetWidth::V4)
            // set border radius
            .set_border_radius(BorderRadius::B(RadiusEnum::Md))
            ;
        println!("class = {}", class.get_class_name())
    }
}
