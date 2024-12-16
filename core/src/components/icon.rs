use yew::prelude::*;

use crate::components::{svg::get_svg, theme::get_color};

use super::{
    svg::IconName,
    theme::{Colors, SizeStyle},
};

#[derive(Properties, PartialEq, Debug)]
pub struct IconProps {
    #[prop_or_default]
    pub name: IconName,

    #[prop_or_default]
    pub size: SizeStyle,

    #[prop_or_default]
    pub color: Colors,

    #[prop_or_default]
    pub width: i32,

    #[prop_or_default]
    pub height: i32,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let width = if props.width.clone() == 0 {
        32
    } else {
        props.width.clone()
    };
    let height = if props.height.clone() == 0 {
        32
    } else {
        props.height.clone()
    };

    let colors = get_color(props.color.clone());

    html! {
        <img src={get_svg(props.name.clone(), width, height, colors)} class="text-blue-800" alt="Icon show failed" />
    }
}
