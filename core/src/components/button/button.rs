use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::svg::IconName;
use crate::components::theme::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct ButtonProps {
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
    pub class: Option<String>,

    #[prop_or_default]
    pub block: bool,

    #[prop_or_default]
    pub link: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub icon: Option<IconName>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let click_handler = props.click.clone();
    let click = Callback::from(move |_event: MouseEvent| {
        click_handler.emit(());
    });

    let mut type_style = if props.link {
        match props.r#type {
            TypeStyle::Primary => "text-blue-600 hover:text-blue-300".to_string(),
            TypeStyle::Warning => "text-orange-500 hover:text-orange-300".to_string(),
            TypeStyle::Danger => "text-red-600 hover:text-red-300".to_string(),
            TypeStyle::Success => "text-green-600 hover:text-green-300".to_string(),
            TypeStyle::Info => "text-sky-600 hover:text-sky-300".to_string(),
            TypeStyle::Default => "text-gray-600 hover:text-gray-300".to_string(),
        }
    } else {
        match props.r#type {
            TypeStyle::Primary => {
                "bg-blue-500 hover:bg-blue-600 hover:border-blue-700 text-white".to_string()
            }
            TypeStyle::Warning => {
                "bg-orange-500 hover:bg-orange-600 hover:border-orange-700 text-white".to_string()
            }
            TypeStyle::Danger => {
                "bg-red-500 hover:bg-red-600 hover:border-red-700 text-white".to_string()
            }
            TypeStyle::Success => {
                "bg-green-500 hover:bg-green-600 hover:border-green-700 text-white".to_string()
            }
            TypeStyle::Info => {
                "bg-sky-400 hover:bg-sky-500 hover:border-sky-700 text-white".to_string()
            }
            TypeStyle::Default => "bg-gray-100 hover:bg-gray-200 hover:border-gray-300".to_string(),
        }
    };

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
    };

    match props.rounded {
        Rounded::Full => {
            type_style = format!("{} rounded-full px-2 py-2", type_style);
        }
        Rounded::Xl => {
            type_style = format!("{} rounded-xl px-2 py-2 ", type_style);
        }
        Rounded::Xl2 => {
            type_style = format!("{} rounded-2xl px-2 py-2", type_style);
        }
        Rounded::Xl3 => {
            type_style = format!("{} rounded-3xl px-2 py-2", type_style);
        }
        Rounded::Lg => {
            type_style = format!("{} rounded-lg px-2 py-2", type_style);
        }
        Rounded::Rounded => {
            type_style = format!("{} rounded-md px-2 py-2", type_style);
        }
        Rounded::Sm => {
            type_style = format!("{} rounded-sm px-2 py-2", type_style);
        }
        Rounded::None => {
            type_style = format!("{} rounded-none px-2 py-2", type_style);
        }
    }

    if props.disabled {
        type_style = format!("{} opacity-50 cursor-not-allowed", type_style);
    }

    let color = if props.r#type == TypeStyle::Default {
        Colors::Custom("#999".to_string())
    } else {
        Colors::Custom("#fff".to_string())
    };

    let class = props.class.clone().unwrap_or("".to_string());
    if !class.is_empty() {
        type_style = format!("{} {}", type_style, class);
    }

    html! {
        <button onclick={click} disabled={props.disabled} class={type_style}>
            <div class="flex items-center justify-center">
            if props.icon.is_some() {
                <span class="px-1">
                    <Icon name={props.icon.clone().unwrap()} color={color} width={14} height={14} />
                </span>
            }
            { props.text.clone().unwrap_or("".to_string()) }
            </div>
        </button>
    }
}
