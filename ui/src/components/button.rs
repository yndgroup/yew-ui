use gloo::console::log;
use yew::prelude::*;

use crate::components::theme::TypeStyle;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub text: Option<String>,

    pub click: Callback<()>,

    #[prop_or_default]
    pub r#type: TypeStyle,

    #[prop_or_default]
    pub custom_class: Option<String>,

    #[prop_or_default]
    pub block: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let click_handler = props.click.clone();
    let click = Callback::from(move |_event: MouseEvent| {
        log!("clicked");
        click_handler.emit(());
    });

    let mut type_style = match props.r#type {
        TypeStyle::Primary => "bg-blue-500 hover:bg-blue-600 hover:border-blue-700 text-white font-bold py-2 px-4 rounded".to_string(),
        TypeStyle::Warning => "bg-orange-500 hover:bg-orange-600 hover:border-orange-700 text-white font-bold py-2 px-4 rounded".to_string(),
        TypeStyle::Danger => "bg-red-500 hover:bg-red-600 hover:border-red-700 text-white font-bold py-2 px-4 rounded".to_string(),
        TypeStyle::Success => "bg-green-500 hover:bg-green-600 hover:border-green-700 text-white font-bold py-2 px-4 rounded".to_string(),
        TypeStyle::Info => "bg-sky-400 hover:bg-sky-500 hover:border-sky-700 text-white font-bold py-2 px-4 rounded".to_string(),
        TypeStyle::Default => "bg-gray-100 hover:bg-gray-200 hover:border-gray-300 font-bold py-2 px-4 rounded".to_string(),
    };
    
    let custom_class = props.custom_class.clone().unwrap_or("".to_string());
    if !custom_class.is_empty() {
        type_style = format!("{} {}", type_style, custom_class);
    }

    if props.block {
        type_style = format!("{} w-full", type_style);
    }

    html! {
        <button onclick={click} class={type_style}>{ format!("{}", props.text.clone().unwrap_or("按钮".to_string())) }</button>
    }
}
