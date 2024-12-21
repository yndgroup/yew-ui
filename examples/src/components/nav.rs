use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    return html! {
        <nav class="flex justify-between border-b-2 p-4">
            <div>
                <a class="hover:text-red-400 font-bold text-2xl" href="/">{"YEW-UI"}</a>
            </div>
            <div class="flex gap-4">
                <a class="hover:text-red-400" href="/docs">{"Docs"}</a>
                <a  class="hover:text-red-400" href="https://github.com/yndgroup/yew-ui" target="_blank">{"github"}</a>
            </div>
        </nav>
    };
}