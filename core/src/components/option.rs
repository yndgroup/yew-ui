use yew::prelude::*;

#[derive(PartialEq, Clone)] 
pub struct OptionItem {
    pub name: String,
    pub value: String,
}

#[derive(Properties, PartialEq)]
pub struct OptionProps {
    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub value: String,
}

#[function_component(Option)]
pub fn option(props: &OptionProps) -> Html {
    html! {
        <option value={format!("{}", props.value)}>{format!("{}", props.name) }</option>
    }
}
