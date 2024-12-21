use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub prev: String,

    #[prop_or_default]
    pub next: String,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    return html! {
        <nav class="flex justify-between text-green-600 p-4 mt-4 text-2xl">
            <div>
                <a href={props.prev.clone()}>{"Prev"}</a>
            </div>
            <div>
                <a href={props.next.clone()}>{"Next"}</a>
            </div>
        </nav>
    };
}
