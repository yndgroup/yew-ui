use yew::prelude::*;

#[function_component(PageHome)]
pub fn page_home() -> Html {
    html! {
        <div class="home">
            <h1 class="text-4xl text-center pt-5">{"YewUI = Yew + Tailwindcss"}</h1>
        </div>
    }
}
