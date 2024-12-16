use yew::prelude::*;

use crate::components::theme::TypeStyle;

use super::theme::{Rounded, SizeStyle};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub text: Option<String>,

    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub r#type: TypeStyle,

    #[prop_or_default]
    pub size: SizeStyle,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub custom_class: Option<String>,

    #[prop_or_default]
    pub block: bool,

    #[prop_or_default]
    pub link: bool,

    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let click_handler = props.click.clone();
    let click = Callback::from(move |_event: MouseEvent| {
        click_handler.emit(());
    });

    let mut type_style = if props.link {
        match props.r#type {
            TypeStyle::Primary => "text-blue-600 hover:text-blue-300 py-2 px-4".to_string(),
            TypeStyle::Warning => "text-orange-500 hover:text-orange-300 py-2 px-4".to_string(),
            TypeStyle::Danger => "text-red-600 hover:text-red-300 py-2 px-4".to_string(),
            TypeStyle::Success => "text-green-600 hover:text-green-300 py-2 px-4".to_string(),
            TypeStyle::Info => "text-sky-600 hover:text-sky-300 py-2 px-4".to_string(),
            TypeStyle::Default => "text-gray-600 hover:text-gray-300 py-2 px-4".to_string(),
        }
    } else {
        match props.r#type {
            TypeStyle::Primary => {
                "bg-blue-500 hover:bg-blue-600 hover:border-blue-700 text-white py-2 px-4"
                    .to_string()
            }
            TypeStyle::Warning => {
                "bg-orange-500 hover:bg-orange-600 hover:border-orange-700 text-white py-2 px-4"
                    .to_string()
            }
            TypeStyle::Danger => {
                "bg-red-500 hover:bg-red-600 hover:border-red-700 text-white py-2 px-4".to_string()
            }
            TypeStyle::Success => {
                "bg-green-500 hover:bg-green-600 hover:border-green-700 text-white py-2 px-4"
                    .to_string()
            }
            TypeStyle::Info => {
                "bg-sky-400 hover:bg-sky-500 hover:border-sky-700 text-white py-2 px-4".to_string()
            }
            TypeStyle::Default => {
                "bg-gray-100 hover:bg-gray-200 hover:border-gray-300 py-2 px-4".to_string()
            }
        }
    };

    let custom_class = props.custom_class.clone().unwrap_or("".to_string());
    if !custom_class.is_empty() {
        type_style = format!("{} {}", type_style, custom_class);
    }

    if props.block {
        type_style = format!("{} w-full", type_style);
    }

    match props.size {
        SizeStyle::Xl9 => {
            type_style = format!("{} text-9xl", type_style);
        }
        SizeStyle::Xl8 => {
            type_style = format!("{} text-8xl", type_style);
        }
        SizeStyle::Xl7 => {
            type_style = format!("{} text-7xl", type_style);
        }
        SizeStyle::Xl6 => {
            type_style = format!("{} text-6xl", type_style);
        }
        SizeStyle::Xl5 => {
            type_style = format!("{} text-5xl", type_style);
        }
        SizeStyle::Xl4 => {
            type_style = format!("{} text-4xl", type_style);
        }
        SizeStyle::Xl3 => {
            type_style = format!("{} text-3xl", type_style);
        }
        SizeStyle::Xl2 => {
            type_style = format!("{} text-2xl", type_style);
        }
        SizeStyle::Xl => {
            type_style = format!("{} text-xl", type_style);
        }
        SizeStyle::Lg => {
            type_style = format!("{} text-lg", type_style);
        }
        SizeStyle::Base => {
            type_style = format!("{} text-base", type_style);
        }
        SizeStyle::Sm => {
            type_style = format!("{} text-sm", type_style);
        }
        SizeStyle::Xs => {
            type_style = format!("{} text-xs", type_style);
        }
        _ => type_style = format!("{} text-base", type_style),
    };

    match props.rounded {
        Rounded::Full => {
            type_style = format!("{} rounded-full", type_style);
        }
        Rounded::Xl => {
            type_style = format!("{} rounded-xl", type_style);
        }
        Rounded::Xl2 => {
            type_style = format!("{} rounded-2xl", type_style);
        }
        Rounded::Xl3 => {
            type_style = format!("{} rounded-3xl", type_style);
        }
        Rounded::Lg => {
            type_style = format!("{} rounded-lg", type_style);
        }
        Rounded::Rounded => {
            type_style = format!("{} rounded-md", type_style);
        }
        Rounded::Sm => {
            type_style = format!("{} rounded-sm", type_style);
        }
        Rounded::None => {
            type_style = format!("{} rounded-none", type_style);
        }
        _ => type_style = format!("{} rounded-none", type_style),
    }

    if props.disabled {
        type_style = format!("{} opacity-50 cursor-not-allowed", type_style);
    }

    html! {
        <button onclick={click} disabled={props.disabled} class={type_style}>{ format!("{}", props.text.clone().unwrap_or("按钮".to_string())) }</button>
    }
}
