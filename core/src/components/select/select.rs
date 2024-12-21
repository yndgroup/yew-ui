use yew::prelude::*;

use super::option::OptionItem;
use super::option::Option;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub data: Vec<OptionItem>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    html! {
        <select>
            {for props.data.iter().map(|item| html! {
                <Option name={item.name.clone()} value={item.value.clone()} />
            })}
        </select>
    }
}